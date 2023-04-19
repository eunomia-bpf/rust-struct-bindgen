use std::path::PathBuf;
#[allow(unused)]
pub(crate) fn get_assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}
