use anyhow::Result;
use btf::types::{Btf, BtfArray, BtfType};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::helper::{func_names_ident, ty_name};

pub(crate) fn generate_binding_for_array(
    btf: &Btf,
    _array: &BtfArray,
    ty_id: u32,
) -> Result<TokenStream> {
    let ty_name_ident = Ident::new(&ty_name(ty_id), Span::call_site());
    let mut dims = vec![];
    let arr_elem_ty = lookup_array(btf, &mut dims, ty_id);
    let elem_ty_ident = Ident::new(&ty_name(arr_elem_ty), Span::call_site());
    // C arrays are int[2][3][4][5], which correponds [[[[ i32 ; 5]; 4]; 4]; 2] in rust
    dims.reverse();
    let mut array_decl = quote! {#elem_ty_ident};
    for dim in dims.iter() {
        let dim_lit = Literal::usize_suffixed(*dim as usize);
        array_decl = quote! {
            [#array_decl; #dim_lit]
        };
    }
    let arr_size_lit = Literal::usize_suffixed(btf.get_size_of(ty_id) as _);
    let (de_func, ser_func) = func_names_ident(ty_id);

    Ok(quote! {
        pub type #ty_name_ident = #array_decl;
        pub fn #de_func (b: &[u8])-> Result<#ty_name_ident, String> {
            let all_val : [u8; #arr_size_lit] = b.try_into().map_err(|_| format!("Slice of size {} expected", #arr_size_lit))?;
            Ok(
                unsafe {std::mem::transmute(all_val)}
            )
        }
        pub fn #ser_func (v: & #ty_name_ident) -> Result<Vec<u8>, String> {
            let all_val : [u8; #arr_size_lit] = unsafe {std::mem::transmute(v.clone())};
            Ok(all_val.to_vec())
        }
    })
}

/// returns the typeid of the element
fn lookup_array(btf: &Btf, dims_out: &mut Vec<u32>, ty_id: u32) -> u32 {
    let mut curr = ty_id;
    loop {
        if let BtfType::Array(arr) = btf.type_by_id(curr) {
            dims_out.push(arr.nelems);
            curr = arr.val_type_id;
        } else {
            break;
        }
    }
    return curr;
}
