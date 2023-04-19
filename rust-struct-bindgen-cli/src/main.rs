use anyhow::{anyhow, Context};
use clap::Parser;
use rust_struct_bindgen_impl::{
    btf::types::Btf, generate_bindgen_token_stream, helper::create_elf_with_btf_section,
    object::ElfFile,
};
#[derive(Parser)]
#[command(about, long_about, version)]
struct Args {
    #[arg(
        help = "The provided file is a plain btf archive",
        short = 'b',
        long = "btf"
    )]
    use_btf: bool,
    #[arg(help = "The ELF file path. If with `use_btf`, should be the btf archive path")]
    file_path: String,
    #[arg(help = "Out file. If not given, print to stdout")]
    out_file: Option<String>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file_buf =
        std::fs::read(&args.file_path).with_context(|| anyhow!("Failed to read input file"))?;
    let elf_bin = if args.use_btf {
        create_elf_with_btf_section(&file_buf, true)
            .with_context(|| anyhow!("Failed to convert BTF into ELF"))?
    } else {
        file_buf
    };
    let elf = ElfFile::parse(&elf_bin).map_err(|e| anyhow!("Failed to parse ELF: {}", e))?;
    let btf = Btf::load(&elf).map_err(|e| anyhow!("Failed to parse BTF: {}", e))?;
    let generated_source = generate_bindgen_token_stream(&btf)
        .with_context(|| anyhow!("Failed to generate rust code"))?
        .to_string();
    if let Some(p) = args.out_file {
        std::fs::write(p, generated_source).with_context(|| anyhow!("Failed to write"))?;
    } else {
        print!("{}", generated_source);
    }
    Ok(())
}
