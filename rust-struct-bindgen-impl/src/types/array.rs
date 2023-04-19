use anyhow::Result;
use btf::types::{Btf, BtfArray, BtfIntEncoding, BtfType};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::{
    cache::SizeResolveCache,
    helper::{func_names_ident, ty_name},
};

pub(crate) fn generate_binding_for_array(
    btf: &Btf,
    array: &BtfArray,
    ty_id: u32,
    size_resolver: &mut SizeResolveCache,
) -> Result<TokenStream> {
    let ty_name_ident = Ident::new(&ty_name(ty_id), Span::call_site());
    let (de_func, ser_func) = func_names_ident(ty_id);
    let elem_count_lit = Literal::usize_suffixed(array.nelems as usize);
    if is_char_array(btf, array) {
        // For char arrays, treat them as strings
        Ok(quote! {
            #[allow(unused)]
            #[allow(non_camel_case_types)]
            pub type #ty_name_ident = String;
            #[allow(unused)]
            pub fn #de_func(b: &[u8]) -> Result<#ty_name_ident, String> {
                let mut idx = 0;
                while idx < b.len() && b[idx] != 0{
                    idx += 1;
                }
                if idx == b.len(){
                    return Err("zero byte not found when deserializing".to_string());
                }
                String::from_utf8(b[..idx].to_vec()).map_err(|e|format!("Invalid utf8 strings when deserializling: {}",e))
            }
            #[allow(unused)]
            pub fn #ser_func(v: & #ty_name_ident) -> Result<Vec<u8>,String> {
                let mut bytes = v.as_bytes().to_vec();
                bytes.push(0);
                if bytes.len() > #elem_count_lit {
                    return Err(format!("String is too long! only {} bytes is allowed", #elem_count_lit - 1));
                }
                while bytes.len() < #elem_count_lit {
                    bytes.push(0);
                }
                Ok(bytes)
            }
        })
    } else {
        let elem_ty_ident = Ident::new(&ty_name(array.val_type_id), Span::call_site());

        let elem_size_lit = Literal::usize_suffixed(size_resolver.resolve(array.val_type_id));
        let array_decl = quote! {
            [#elem_ty_ident; #elem_count_lit]
        };
        let (el_de_func, el_ser_func) = func_names_ident(array.val_type_id);
        Ok(quote! {
            #[allow(unused)]
            #[allow(non_camel_case_types)]
            pub type #ty_name_ident = #array_decl;
            #[allow(unused)]
            #[allow(clippy::needless_range_loop)]
            pub fn #de_func (b: &[u8])-> Result<#ty_name_ident, String> {
                let mut result = vec![];
                for i in 0..#elem_count_lit {
                    result.push(#el_de_func ( &b[i* #elem_size_lit .. (i+1) * #elem_size_lit])?);
                }
                Ok(result.try_into().unwrap())
            }
            #[allow(unused)]
            #[allow(clippy::needless_range_loop)]
            pub fn #ser_func (v: & #ty_name_ident) -> Result<Vec<u8>, String> {
                let mut result = vec![];
                for i in 0..#elem_count_lit {
                    result.extend(#el_ser_func ( &v[i] )? .into_iter());
                }
                Ok(result)
            }
        })
    }
}

fn is_char_array(btf: &Btf, array: &BtfArray) -> bool {
    is_char(btf, array.val_type_id)
}

fn is_char(btf: &Btf, ty_id: u32) -> bool {
    match btf.type_by_id(ty_id) {
        BtfType::Int(btf_int) => {
            (btf_int.name.ends_with("char") || matches!(btf_int.encoding, BtfIntEncoding::Char))
                && btf_int.bits == 8
        }
        _ => false,
    }
}
