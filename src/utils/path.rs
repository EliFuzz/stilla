use dirs::home_dir;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub fn plugins_dir() -> &'static Path {
    static PLUGINS_DIR: OnceLock<PathBuf> = OnceLock::new();

    PLUGINS_DIR
        .get_or_init(|| home_dir().unwrap().join(".stilla"))
        .as_path()
}

pub fn ensure_plugins_dir() {
    create_dir_all(plugins_dir()).ok();
}
