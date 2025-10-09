use std::fs;
use std::path::Path;

use crate::libs::config::{Config};

pub fn ensure_dirs(config: &Config) -> std::io::Result<()> {
    let dirs = [
        &config.base_dir,
        &config.bin_dir,
        &config.conf_dir,
        &config.repo_dir,
    ];

    for dir in dirs {
        create_dir(dir)?;
    }

    Ok(())
}

fn create_dir(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        log::info!("creating directory: {}", path.display());
        fs::create_dir(path)?;
    } else {
        log::debug!("directory already exists: {}", path.display());
    }
    Ok(())
}
