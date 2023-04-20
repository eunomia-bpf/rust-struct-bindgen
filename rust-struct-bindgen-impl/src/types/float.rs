//!  SPDX-License-Identifier: MIT
//!
//! Copyright (c) 2023, eunomia-bpf
//! All rights reserved.
//!
use anyhow::{bail, Result};
use btf::types::{Btf, BtfFloat};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::helper::{func_names_ident, ty_name};

pub(crate) fn generate_binding_for_float(
    _btf: &Btf,
    btf_float: &BtfFloat,
    ty_id: u32,
) -> Result<TokenStream> {
    let ty_name_ident = Ident::new(&ty_name(ty_id), Span::call_site());
    let (de_func, ser_func) = func_names_ident(ty_id);
    let underlying_type_ident = Ident::new(
        match btf_float.sz {
            4 => "f32",
            8 => "f64",
            s => bail!("Unsupported float size: {}", s),
        },
        Span::call_site(),
    );
    let size_lit = Literal::usize_suffixed(btf_float.sz as usize);

    Ok(quote! {
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub type #ty_name_ident = #underlying_type_ident;
        #[allow(unused)]
        pub fn #de_func (b:&[u8]) -> Result<#ty_name_ident, String> {
            if b.len() != #size_lit {
                return Err(format!("Expected a slice in {} bytes", #size_lit));
            }
            Ok(
                #underlying_type_ident :: from_ne_bytes(b.try_into().unwrap())
            )
        }
        #[allow(unused)]
        pub fn #ser_func(v: &#ty_name_ident) -> Result<Vec<u8>, String> {
            Ok(
                v.to_ne_bytes().to_vec()
            )
        }
    })
}
