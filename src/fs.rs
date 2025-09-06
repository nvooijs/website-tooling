use crate::Result;
use crate::path::Path;

pub use cap_std::fs_utf8::{Dir, File};

pub fn copy(from_dir: &Dir, from: &Path, to_dir: &Dir, to: &Path) -> Result<()> {
    if let Some(p) = to.parent() {
        to_dir.create_dir_all(p)?;
    }
    from_dir.copy(from, to_dir, to)?;
    Ok(())
}

pub fn create(dir: &Dir, path: &Path) -> Result<File> {
    if let Some(p) = path.parent() {
        dir.create_dir_all(p)?;
    }
    let file = dir.create(path)?;
    Ok(file)
}
