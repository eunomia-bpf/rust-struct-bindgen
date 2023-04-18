use anyhow::Result;
use btf::types::{Btf, BtfType};
use proc_macro2::TokenStream;

pub use btf;
pub use object;
use types::{
    array::generate_binding_for_array, float::generate_binding_for_float,
    integer::generate_binding_for_integer, structure::generate_binding_for_struct,
};
pub(crate) mod helper;
pub(crate) mod types;
pub fn generate_bindgen_token_stream(btf: &Btf) -> Result<TokenStream> {
    let mut result = TokenStream::default();
    for (ty_id, ty) in btf.types().iter().enumerate().map(|v| (v.0 as u32, v.1)) {
        match ty {
            BtfType::Struct(comp) => {
                result.extend(generate_binding_for_struct(btf, comp, ty_id)?);
            }
            BtfType::Int(btf_int) => {
                result.extend(generate_binding_for_integer(btf, btf_int, ty_id)?);
            }
            BtfType::Array(array) => {
                result.extend(generate_binding_for_array(btf, array, ty_id)?);
            }
            BtfType::Float(ft) => result.extend(generate_binding_for_float(btf, ft, ty_id)?),
            _ => continue,
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn test_1() {
        let mut s = quote! {
            struct s1 {
                a:i32,
                b:i64
            }
            impl s1 {
                pub fn f1(&self) -> i64 {
                    self.a as i64 + self.b
                }
            }
        };
        s.extend(quote! {struct s2(i32,i32);});
        println!("{}", s.to_string());
    }
}
