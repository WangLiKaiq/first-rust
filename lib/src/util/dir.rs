use std::path::PathBuf;

pub fn get_project_root() -> std::io::Result<PathBuf> {
    let root = std::env::current_dir().unwrap();
    Ok(root)
}
