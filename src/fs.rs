use crate::Result;
use crate::path::Path;
use std::io::Read;

pub use cap_std::fs_utf8::{Dir, File};

type Bytes = Vec<u8>;

pub fn read_file_bytes(file: &mut File) -> Result<Bytes> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn read_file_str(file: &mut File) -> Result<String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn create(dir: &Dir, path: &Path) -> Result<File> {
    if let Some(p) = path.parent() {
        dir.create_dir_all(p)?;
    }
    let file = dir.create(path)?;
    Ok(file)
}
