use std::path::PathBuf;

use proc_macro::TokenStream;
use rust_struct_bindgen_impl::{btf::types::Btf, generate_bindgen_token_stream, object::ElfFile};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn btf_struct_bindgen_with_elf(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr);
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let btf_file_path = root.join(file_path.value());
    let buf = std::fs::read(&btf_file_path).expect(&format!(
        "Failed to read the btf file {}",
        btf_file_path.to_str().unwrap()
    ));
    let object = ElfFile::parse(&buf).expect("Failed to parse file as ELF");
    let btf_data = Btf::load(&object).expect("Failed to parse BTF");
    let stream = generate_bindgen_token_stream(&btf_data).unwrap();
    return stream.into();
}

#[proc_macro]
pub fn btf_struct_bindgen_with_btf(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr);
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let btf_file_path = root.join(file_path.value());
    let buf = std::fs::read(&btf_file_path).expect(&format!(
        "Failed to read the btf file {}",
        btf_file_path.to_str().unwrap()
    ));
    let object = ElfFile::parse(&buf).expect("Failed to parse file as ELF");
    let btf_data = Btf::load(&object).expect("Failed to parse BTF");
    let stream = generate_bindgen_token_stream(&btf_data).unwrap();
    return stream.into();
}
