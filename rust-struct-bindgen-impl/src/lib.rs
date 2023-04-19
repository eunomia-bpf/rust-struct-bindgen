use anyhow::Result;
use btf::types::{Btf, BtfType};
use cache::SizeResolveCache;
use proc_macro2::TokenStream;

pub use btf;
pub use object;

use quote::quote;
use types::{
    array::generate_binding_for_array, enumeration::generate_binding_for_enum,
    float::generate_binding_for_float, generate_binding_for_pointer,
    integer::generate_binding_for_integer, structure::generate_binding_for_struct,
};
pub(crate) mod cache;
/// Some helper functions
pub mod helper;
pub(crate) mod types;

pub fn generate_bindgen_token_stream(btf: &Btf) -> Result<TokenStream> {
    let mut inner_impl = TokenStream::new();
    let mut outer_impl = TokenStream::new();
    let mut size_cache = SizeResolveCache::new(btf);
    for (ty_id, ty) in btf.types().iter().enumerate().map(|v| (v.0 as u32, v.1)) {
        match ty {
            BtfType::Struct(comp) => {
                let (outer, inner) =
                    generate_binding_for_struct(btf, comp, ty_id, &mut size_cache)?;
                inner_impl.extend(inner);
                outer_impl.extend(outer);
            }
            BtfType::Int(btf_int) => {
                inner_impl.extend(generate_binding_for_integer(
                    btf,
                    btf_int,
                    ty_id,
                    &mut size_cache,
                )?);
            }
            BtfType::Array(array) => {
                inner_impl.extend(generate_binding_for_array(
                    btf,
                    array,
                    ty_id,
                    &mut size_cache,
                )?);
            }
            BtfType::Float(ft) => inner_impl.extend(generate_binding_for_float(btf, ft, ty_id)?),
            BtfType::Enum(btf_enum) => {
                let (outer, inner) = generate_binding_for_enum(btf, btf_enum, ty_id)?;
                inner_impl.extend(inner);
                outer_impl.extend(outer);
            }
            BtfType::Ptr(_) => {
                inner_impl.extend(generate_binding_for_pointer(btf, ty_id)?);
            }
            _ => continue,
        }
    }

    Ok(quote! {
        #[allow(unused)]
        pub mod inner_impl {
            #inner_impl
        }
        #outer_impl
    })
}
