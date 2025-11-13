use std::path::{Path, PathBuf};

use anyhow::Result;

pub trait Process {
    fn process(&self, path: &Path, file_name: &Path) -> Result<PathBuf>;
}
