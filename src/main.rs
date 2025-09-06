type Result<T> = anyhow::Result<T>;

mod path {
    pub use camino::{Utf8Path as Path, Utf8PathBuf as PathBuf};
}
mod fs;

mod cli;
mod config;
mod hash;

use crate::path::*;
use anyhow::{Context, anyhow};
use std::collections::HashMap;

fn open_root_dir(dir: Option<&Path>) -> Result<fs::Dir> {
    let aa = cap_std::ambient_authority();
    Ok(match dir {
        Some(path) => fs::Dir::open_ambient_dir(path, aa)?,
        None => fs::Dir::open_ambient_dir(".", aa)?,
    })
}

fn program() -> Result<()> {
    let cli = cli::parse();

    let root = open_root_dir(cli.dir.as_ref().map(|p| p.as_ref()))?;

    let config = config::Config::open(&root, cli.config.as_ref()).context("opening config file")?;

    let mut mapping = HashMap::<String, String>::with_capacity(config.assets.len());

    for asset in config.assets.iter() {
        let source: &Path = asset.source.as_ref();

        let hash = {
            let file = root.open(source)?;
            hash::ShortHash::from_file(&file)
        };

        let out_dir = if let Some(p) = config.out_dir.as_ref() {
            root.create_dir_all(p)?;
            root.open_dir(p).context("opening output directory")?
        } else {
            root.try_clone().context("reopening root directory")?
        };

        let mut target = PathBuf::new();
        target.push(asset.target_dir.as_ref().unwrap_or(&Default::default()));

        let (name, target_name) = {
            let stem = asset
                .name
                .as_ref()
                .map(|s| s.as_ref())
                .or_else(|| source.file_stem())
                .ok_or(anyhow!("source has no file stem"))?;
            let ext = source.extension().unwrap();
            (format!("{stem}.{ext}"), format!("{stem}-{hash}.{ext}"))
        };
        target.push(target_name);

        fs::copy(&root, source, &out_dir, target.as_ref())
            .context("copying asset to destination")?;

        mapping.insert(name, target.to_string());
    }

    let mut path = PathBuf::new();
    path.push(config.out_dir.as_ref().unwrap_or(&Default::default()));
    path.push("mapping.json");
    let file = fs::create(&root, path.as_ref()).context("creating mapping file")?;
    serde_json::to_writer_pretty(&file, &mapping).context("serializing and writing mapping")?;

    Ok(())
}

fn main() {
    // TODO: handle errors
    program().expect("error")
}
