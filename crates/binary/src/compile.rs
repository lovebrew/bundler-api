use std::path::{Path, PathBuf};

use crate::metadata::Metadata;

use anyhow::Result;

pub trait Compile {
    fn compile(&self, path: &Path, metadata: &Metadata, icon: &Path) -> Result<(PathBuf, Vec<u8>)>;
}
