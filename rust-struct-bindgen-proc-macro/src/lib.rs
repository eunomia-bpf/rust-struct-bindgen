use proc_macro::TokenStream;
use rust_struct_bindgen_impl::{
    btf::types::Btf, generate_bindgen_token_stream, helper::create_elf_with_btf_section,
    object::ElfFile,
};
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};
/// Generate binding source codes for the BTF info in the provided ELF
///
/// The calling syntax should be:
/// ```rust
/// btf_struct_bindgen_with_elf!("xxx.bpf.o");
/// ```
///
/// Where `xxx.bpf.o` is file path relatived to the `CARGO_MANIFEST_DIR`, aka the directory where `Cargo.toml` of you project exists
#[proc_macro]
pub fn btf_struct_bindgen_with_elf(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr);
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let btf_file_path = root.join(file_path.value());
    let buf = std::fs::read(&btf_file_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read the elf file {}",
            btf_file_path.to_str().unwrap()
        )
    });
    let object = ElfFile::parse(&buf).expect("Failed to parse file as ELF");
    let btf_data = Btf::load(&object).expect("Failed to parse BTF");
    let stream = generate_bindgen_token_stream(&btf_data).unwrap();
    stream.into()
}
/// Generate binding source codes for the provided BTF archive
///
/// The calling syntax should be:
/// ```rust
/// btf_struct_bindgen_with_btf!("xxx.btf");
/// ```
///
/// Where `xxx.bpf.o` is file path relatived to the `CARGO_MANIFEST_DIR`, aka the directory where `Cargo.toml` of you project exists
#[proc_macro]
pub fn btf_struct_bindgen_with_btf(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr);
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let btf_file_path = root.join(file_path.value());
    let buf = std::fs::read(&btf_file_path).unwrap_or_else(|_| {
        panic!(
            "Failed to read btf file {}",
            btf_file_path.to_str().unwrap()
        )
    });
    let buf = create_elf_with_btf_section(&buf[..], true).unwrap();
    let object = ElfFile::parse(&buf).expect("Failed to parse file as ELF");
    let btf_data = Btf::load(&object).expect("Failed to parse BTF");
    let stream = generate_bindgen_token_stream(&btf_data).unwrap();
    stream.into()
}
