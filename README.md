# rust-struct-bindgen

Here the repo of `rust-struct-bindgen`, a rust source code generator to read & write native structs with BTF.

There are three crates:
- `rust-struct-bindgen-impl`: The core implementation, which accepts a `btf::types::Btf` and yields a `TokenStream` represented rust source code.
- `rust-struct-bindgen-proc-macro`: A wrapper for the `impl`, provides convenience for using `rust-struct-bindgen` in rust sources.
- `rust-struct-bindgen-cli`: Another wrapper. Which accepts btf file path from command line argument and prints the output source code to stdout or writes to file

For detailed docs, refer to the doc in `lib.rs` of `rust-struct-bindgen-impl`

# Build & Usage

Only the CLI crate can be used seperately. To use that, simply run `cargo run` or `cargo build`.

If you prefer, you can also install the CLI with running `cargo install --path .` in the `rust-struct-bindgen-cli` directory.

```console
Usage: rust-struct-bindgen-cli [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  The ELF file path. If with `use_btf`, should be the btf archive path

Options:
  -b, --btf          The provided file is a plain btf archive
  -f, --format       Formatted the generated code. Requires the installation of `rustfmt`
  -o <OUT_FILE>      Out file. If not given, print to stdout
  -h, --help         Print help
  -V, --version      Print version
```

For example, you can invoke the CLI in the following syntax:

```console
rust-struct-bindgen-cli simple_prog.bpf.o -f -o dump.rs
```

Will generate bindings for `simple_prog.bpf.o` (which is an ELF file) , format the generated sources, and write the result to `dump.rs`.
