use rust_struct_bindgen_impl::{object::ElfFile, btf::types::Btf, generate_bindgen_token_stream};

mod bindgen {
    use rust_struct_bindgen_proc_macro::btf_struct_bindgen_with_elf;

    // btf_struct_bindgen_with_elf!("simple_prog.bpf.o");
}

fn main() {
    let buf = std::fs::read("simple_prog.bpf.o").unwrap();
    let elf = ElfFile::parse(&buf).unwrap();
    let btf = Btf::load(&elf).unwrap();
    let ts = generate_bindgen_token_stream(&btf).unwrap();
    println!("{}",ts.to_string());
}
