use std::path::PathBuf;

use anyhow::Result;

pub mod artifact;
pub mod compile;
pub mod convert;
pub mod health;

pub fn artifacts_dir() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let path = current_dir.join(".artifacts");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}
