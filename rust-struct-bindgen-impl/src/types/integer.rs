use anyhow::{bail, Result};
use btf::types::{Btf, BtfInt, BtfIntEncoding};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::{
    cache::SizeResolveCache,
    helper::{func_names_ident, ty_name},
};
pub(crate) fn generate_binding_for_integer(
    _btf: &Btf,
    btf_int: &BtfInt,
    ty_id: u32,
    size_resolver: &mut SizeResolveCache,
) -> Result<TokenStream> {
    if btf_int.bits % 8 != 0 {
        bail!("Bitfield is not supported now");
    }

    let underlying_type_ident = Ident::new(
        match (btf_int.bits, btf_int.encoding) {
            (8, BtfIntEncoding::Bool) => "bool",
            (8, BtfIntEncoding::Signed) => "i8",
            (8, _) => "u8",
            (16, BtfIntEncoding::Signed) => "i16",
            (16, BtfIntEncoding::None) => "u16",
            (32, BtfIntEncoding::Signed) => "i32",
            (32, BtfIntEncoding::None) => "u32",
            (64, BtfIntEncoding::Signed) => "i64",
            (64, BtfIntEncoding::None) => "u64",
            (128, BtfIntEncoding::Signed) => "i128",
            (128, BtfIntEncoding::None) => "u128",
            (a, b) => bail!("Unsupported integer bits {} and encoding {} pair", a, b),
        },
        Span::call_site(),
    );
    let type_size_lit = Literal::usize_suffixed(size_resolver.resolve(ty_id));
    let type_name_ident = Ident::new(&ty_name(ty_id), Span::call_site());
    let (de_func, ser_func) = func_names_ident(ty_id);
    let de_impl = if matches!(btf_int.encoding, BtfIntEncoding::Bool) {
        quote! {
            b[0] == 1
        }
    } else {
        quote! {
            #underlying_type_ident :: from_ne_bytes (b.try_into().unwrap())
        }
    };
    let ser_impl = if matches!(btf_int.encoding, BtfIntEncoding::Bool) {
        quote! {
            vec![ if v {1} else {0} ]
        }
    } else {
        quote! {
            v.to_ne_bytes().to_vec()
        }
    };
    Ok(quote! {
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub type #type_name_ident = #underlying_type_ident;
        #[allow(unused)]
        pub fn #de_func (b: &[u8]) -> Result< #type_name_ident, String> {
            if b.len() != #type_size_lit {
                return Err(format!("Expected a slice with {} bytes", #type_size_lit))
            }
            Ok(
                #de_impl
            )
        }
        #[allow(unused)]
        pub fn #ser_func (v: & #type_name_ident) -> Result< Vec<u8>, String> {
            Ok(
                #ser_impl
            )
        }

    })
}
