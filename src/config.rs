use crate::Result;
use crate::fs::Dir;
use crate::path::{Path, PathBuf};

#[derive(serde::Deserialize)]
pub struct Config {
    /// Directory to output assets
    pub out_dir: Option<PathBuf>,
    /// Files to compute hashes of
    pub assets: Vec<AssetConfig>,
}

/// Configuration for a specific asset
#[derive(serde::Deserialize)]
pub struct AssetConfig {
    /// Asset name, source path name if `None`
    pub name: Option<String>,
    /// Asset source path
    pub source: PathBuf,
    /// File type
    pub r#type: Option<AssetType>,
    /// Target directory to copy the file to
    pub target_dir: Option<PathBuf>,
}

/// Asset type
#[derive(serde::Deserialize)]
pub enum AssetType {
    Html,
    JsScript,
    JsModule,
    Css,
}

impl Config {
    pub fn open(root: &Dir, path: &Path) -> Result<Self> {
        let s = root.read_to_string(path)?;
        Ok(toml::from_str(s.as_ref())?)
    }
}
