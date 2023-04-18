use anyhow::Result;
use btf::types::{Btf, BtfFloat};
use proc_macro2::TokenStream;

pub(crate) fn generate_binding_for_float(
    btf: &Btf,
    btf_float: &BtfFloat,
    ty_id: u32,
) -> Result<TokenStream> {
    todo!();
}
