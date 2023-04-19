use faerie::{ArtifactBuilder, Decl, SectionKind};
use proc_macro::TokenStream;
use rust_struct_bindgen_impl::{btf::types::Btf, generate_bindgen_token_stream, object::ElfFile};
use std::path::PathBuf;
use std::str::FromStr;
use syn::{parse_macro_input, LitStr};
use target_lexicon::triple;
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
    let buf = create_elf_with_btf_section(&buf[..], true);
    let object = ElfFile::parse(&buf).expect("Failed to parse file as ELF");
    let btf_data = Btf::load(&object).expect("Failed to parse BTF");
    let stream = generate_bindgen_token_stream(&btf_data).unwrap();
    stream.into()
}

/// Currently, btfdump doesn't support load BTF from a btf archive
/// So if we want to use btf archive, we have to wrap that into an ELF..
fn create_elf_with_btf_section(btf_data: &[u8], is_64: bool) -> Vec<u8> {
    let mut obj = ArtifactBuilder::new(if is_64 {
        triple!("x86_64-unknown-unknown-unknown-elf")
    } else {
        triple!("i386-unknown-unknown-unknown-elf")
    })
    .name("btf-archive.bpf.o".into())
    .finish();
    obj.declare(".BTF", Decl::section(SectionKind::Data))
        .expect("Failed to build ELF from BTF");
    obj.define(".BTF", btf_data.to_vec())
        .expect("Failed to build ELF from BTF");
    obj.emit().expect("Failed to build ELF from BTF")
}
