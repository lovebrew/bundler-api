use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

use crate::{compile::Compile, metadata::Metadata};
use system::programs;
use system::resources;

pub struct Ctr;

impl Compile for Ctr {
    fn compile(&self, path: &Path, metadata: &Metadata, icon: &Path) -> Result<(PathBuf, Vec<u8>)> {
        println!("{path:?} {icon:?}");
        Ok((PathBuf::new(), Vec::new()))
    }
}
