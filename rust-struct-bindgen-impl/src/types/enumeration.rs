//!  SPDX-License-Identifier: MIT
//!
//! Copyright (c) 2023, eunomia-bpf
//! All rights reserved.
//!
use btf::types::{Btf, BtfEnum};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::helper::{func_names_ident, ty_name};
use anyhow::{bail, Result};
pub(crate) fn generate_binding_for_enum(
    _btf: &Btf,
    btf_enum: &BtfEnum,
    ty_id: u32,
) -> Result<(TokenStream, TokenStream)> {
    let ty_name_ident = Ident::new(&ty_name(ty_id), Span::call_site());
    let (de_func, ser_func) = func_names_ident(ty_id);
    let repr_ident = Ident::new(
        match btf_enum.sz {
            1 => "i8",
            2 => "i16",
            4 => "i32",
            8 => "i64",
            s => bail!("Unsupported enum size `{}` in enum `{}`", s, btf_enum.name),
        },
        Span::call_site(),
    );
    let val_size_lit = Literal::usize_suffixed(btf_enum.sz as usize);
    let (field_name_ident, field_value_lit) = {
        let mut r1 = vec![];
        let mut r2 = vec![];
        btf_enum.values.iter().for_each(|mem| {
            r1.push(Ident::new(mem.name, Span::call_site()));
            r2.push(Literal::i64_suffixed(mem.value as i64));
        });
        (r1, r2)
    };
    let enum_name_lit = Literal::string(btf_enum.name);
    let enum_name_ident = Ident::new(btf_enum.name, Span::call_site());

    Ok((
        quote! {
            #[allow(unused)]
            #[allow(non_camel_case_types)]
            pub type #enum_name_ident = inner_impl:: #ty_name_ident;
            impl #enum_name_ident {
                #[allow(unused)]
                pub fn from_bytes(b: &[u8]) -> Result <Self, String> {
                   inner_impl:: #de_func(b)
                }
                #[allow(unused)]
                pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
                    inner_impl:: #ser_func(self)
                }
            }
        },
        quote! {
            #[allow(unused)]
            #[allow(non_camel_case_types)]
            #[repr(#repr_ident)]
            #[derive(Debug, Clone)]
            pub enum #ty_name_ident {
                #(
                    #[allow(non_camel_case_types)]
                    #field_name_ident,
                )*
            }

            #[allow(unused)]
            pub fn #de_func( b:&[u8]) -> Result <#ty_name_ident, String> {
                if b.len() != #val_size_lit {
                    return Err(format!("Expected a slice in {} bytes", #val_size_lit));
                }
                let val = (#repr_ident :: from_ne_bytes(b.try_into().unwrap())) as i64;
                match val {
                    #(
                        #field_value_lit => Ok(#ty_name_ident :: #field_name_ident),
                    )*
                    s => {
                        Err(format!("Invalid enum value {} for enum {}",s,#enum_name_lit))
                    }
                }
            }
            #[allow(unused)]
            pub fn #ser_func (v: & #ty_name_ident) -> Result<Vec<u8>, String> {
                match v {
                    #(
                        #ty_name_ident :: #field_name_ident => Ok((#field_value_lit as #repr_ident) . to_ne_bytes() . to_vec()),
                    )*
                }
            }
        },
    ))
}
