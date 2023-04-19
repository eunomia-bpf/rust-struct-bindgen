use crate::cache::SizeResolveCache;
use crate::helper::{func_names_ident, lookup_types, ty_name};
use anyhow::anyhow;
use anyhow::{bail, Result};
use btf::types::{Btf, BtfComposite};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
pub(crate) fn generate_binding_for_struct(
    btf: &Btf,
    comp: &BtfComposite,
    ty_id: u32,
    size_resolver: &mut SizeResolveCache,
) -> Result<(TokenStream, TokenStream)> {
    let st_name = Ident::new(&ty_name(ty_id), Span::call_site());
    let alias = Ident::new(comp.name, Span::call_site());

    let (
        field_type_idents,
        field_type_de_func_idents,
        field_type_ser_func_idents,
        field_names,
        field_sizes,
        field_offsets,
    ) = {
        let mut r1 = vec![];
        let mut r2 = vec![];
        let mut r3 = vec![];
        let mut r4 = vec![];
        let mut r5 = vec![];
        let mut r6 = vec![];
        comp.members.iter().try_for_each(|v| -> Result<()> {
            if v.bit_size % 8 != 0 || v.bit_offset % 8 != 0 {
                bail!("Bitfield is not supported, currently");
            }
            let type_name = ty_name(lookup_types(btf, v.type_id).map_err(|e| {
                anyhow!(
                    "Failed to lookup type for struct {} field {}: {}",
                    comp.name,
                    v.name,
                    e
                )
            })?);
            r1.push(Ident::new(&type_name, Span::call_site()));
            let (i2, i3) = func_names_ident(v.type_id);
            r2.push(i2);
            r3.push(i3);
            r4.push(Ident::new(&format!("f_{}", v.name), Span::call_site()));
            r5.push(Literal::usize_suffixed(size_resolver.resolve(v.type_id)));
            r6.push(Literal::usize_suffixed((v.bit_offset / 8) as usize));
            Ok(())
        })?;
        (r1, r2, r3, r4, r5, r6)
    };

    let struct_decl = quote! {
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub struct #st_name {
            #(pub #field_names: #field_type_idents),*
        }
    };
    let (der_name, ser_name) = func_names_ident(ty_id);
    let type_size = Literal::usize_suffixed(comp.sz as _);

    let deserialize_func = quote! {
        #[allow(unused)]
        #[allow(clippy::identity_op)]
        pub fn #der_name (b: &[u8]) -> std::result::Result< #st_name, std::string::String> {
            if b.len() != #type_size {
                return Err(format!("Expected a slice with length {}", #type_size))
            }
            #(
                let #field_names = #field_type_de_func_idents ( &b[ #field_offsets .. #field_offsets + #field_sizes ] )?;
            )*
            Ok(
                #st_name {
                    #(
                        #field_names,
                    )*
                }
            )
        }

    };
    let serialize_func = quote! {
        #[allow(unused)]
        #[allow(clippy::identity_op)]
        pub fn #ser_name (t: & #st_name) -> std::result::Result < Vec<u8> , std::string::String> {
            let mut result = vec![0u8; #type_size ];
            #(
                {
                    let ret = #field_type_ser_func_idents ( &t. #field_names)?;
                    result[#field_offsets .. #field_offsets + #field_sizes].copy_from_slice ( &ret[..] );
                }
            )*

            Ok(result)
        }
    };
    let outer_code = quote! {
        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub type #alias = inner_impl :: #st_name;

        impl #alias {
            #[allow(unused)]
            pub fn from_bytes(b: &[u8]) -> Result<Self, String> {
                inner_impl:: #der_name (b)
            }
            #[allow(unused)]
            pub fn to_bytes(&self) -> Result <Vec<u8>, String> {
                inner_impl:: #ser_name (self)
            }
        }
    };
    Ok((
        outer_code,
        quote! {
            #struct_decl
            #deserialize_func
            #serialize_func
        },
    ))
}
