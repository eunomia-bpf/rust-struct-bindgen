use anyhow::anyhow;
use anyhow::Result;
use btf::types::{Btf, BtfConst, BtfRestrict, BtfType, BtfVolatile};
use proc_macro2::{Ident, Span};
use std::fmt::Display;

#[inline]
/// Generate a type name for the specified type id
pub(crate) fn ty_name(ty_id: impl Display) -> String {
    format!("btf_type_{}", ty_id)
}
#[inline]
/// Generate the deserializing function name and serializing function name for the specified type id
pub(crate) fn func_names(ty_id: impl Display) -> (String, String) {
    let ty_name_local = ty_name(ty_id);
    (
        format!("deserialize_{}", ty_name_local),
        format!("serialize_{}", ty_name_local),
    )
}
#[inline]
/// Generate the Ident object for the ser & de function names of the type id
/// (de, ser)
pub(crate) fn func_names_ident(ty_id: impl Display) -> (Ident, Ident) {
    let (des, ser) = func_names(ty_id);
    (
        Ident::new(&des, Span::call_site()),
        Ident::new(&ser, Span::call_site()),
    )
}
/// Lookup a type over const/restrict/volatile attributes
pub(crate) fn lookup_types(btf: &Btf, ty_id: u32) -> Result<u32> {
    let result = match btf
        .types()
        .get(ty_id as usize)
        .ok_or_else(|| anyhow!("Invalid type: {}", ty_id))?
    {
        BtfType::Typedef(btf_typedef) => lookup_types(btf, btf_typedef.type_id)?,
        BtfType::Const(BtfConst { type_id })
        | BtfType::Restrict(BtfRestrict { type_id })
        | BtfType::Volatile(BtfVolatile { type_id }) => lookup_types(btf, *type_id)?,
        _ => ty_id,
    };
    Ok(result)
}
