use std::path::PathBuf;

use anyhow::{Result, bail};
use directories::ProjectDirs;

pub mod artifact;
pub mod compile;
pub mod convert;
pub mod health;

pub fn artifacts_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("org", "lovebrew", "bundler");
    if let Some(dirs) = dirs {
        let path = dirs.data_dir().join("artifacts");
        std::fs::create_dir_all(&path)?;
        return Ok(path);
    }
    bail!("Failed to get project directories.")
}
