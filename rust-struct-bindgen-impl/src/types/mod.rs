//!  SPDX-License-Identifier: MIT
//!
//! Copyright (c) 2023, eunomia-bpf
//! All rights reserved.
//!
use btf::types::Btf;

pub(crate) mod array;
pub(crate) mod enumeration;
pub(crate) mod float;
pub(crate) mod integer;
pub(crate) mod structure;
use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::helper::{func_names_ident, ty_name};
pub(crate) fn generate_binding_for_pointer(_btf: &Btf, ty_id: u32) -> Result<TokenStream> {
    let ty_name_ident = Ident::new(&ty_name(ty_id), Span::call_site());
    let (de_func, ser_func) = func_names_ident(ty_id);

    Ok(quote! {
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub type #ty_name_ident = u64;

        #[allow(unused)]
        pub fn #de_func (b: &[u8]) -> Result< #ty_name_ident, String> {
            if b.len() != 8 {
                return Err("Expected a slice with 8 bytes".to_string());
            }
            Ok(
                u64 :: from_ne_bytes (b.try_into().unwrap())

            )
        }
        #[allow(unused)]
        pub fn #ser_func (v: & #ty_name_ident) -> Result< Vec<u8>, String> {
            Ok(
                v.to_ne_bytes().to_vec()
            )
        }
    })
}
