//!
//! # Bindgen implementation
//! Here is the core implementation of `rust-struct-bindgen`, which accepts a BTF or ELF file and yields the TokenStream-represented rust source.
//!
//! # The generating stragety
//!
//! - For every `struct`, `enum`, `int`, `float`, `array`, `ptr`, `rust-struct-bindgen` will generate a represented type named `btf_type_XX`, where `XX` is the btf type id of the corresponding type for them. And the corresponding serializing and deserializing function will also be generated.  
//! - Serializing functions always have signature like `fn (&T) -> Result<Vec<u8>, String>`where `T` is the generated rust type. The function name will always be `deserialize_btf_type_XX`, where `XX` is the type id. This function will be in the `inner_impl` module.
//! - Deserializing functions have signature like `fn (&[u8]) -> Result<T, String>`, `T` is also the generated rust type. The function name will always be `serialize_btf_type_XX`, where `XX` is the type id. This function will be in the `inner_impl` module.
//! - struct and enums will have a type alias pointing to the `btf_type_XX` type. The alias will be its original name. And deserializing function and serializing function will also be associated functions of the alias type.
//! - The alias of `struct` and `enums` will be put at the top level module. All other type definitions and (de)serializing functions will be put under a module named `inner_impl`; All things are `pub`.
//! e.g, for a btf int with sz 64bits and unsigned encoding and typeid `1`, `rust-struct-bindgen` will generate the following code:
//! ```rust
//!    #[allow(unused)]
//!    #[allow(non_camel_case_types)]
//!    pub type btf_type_1 = u64;
//!    #[allow(unused)]
//!    pub fn deserialize_btf_type_1(b: &[u8]) -> Result<btf_type_1, String> {
//!        if b.len() != 8 {
//!            return Err("Expected a slice with 8 bytes".to_string());
//!        }
//!        Ok(u64::from_ne_bytes(b.try_into().unwrap()))
//!    }
//!    #[allow(unused)]
//!    pub fn serialize_btf_type_1(v: &btf_type_1) -> Result<Vec<u8>, String> {
//!        Ok(v.to_ne_bytes().to_vec())
//!    }
//!```

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
/// Generate a TokenStream for the specified Btf
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
