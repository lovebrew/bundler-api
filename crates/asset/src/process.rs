use std::path::Path;

use anyhow::Result;

pub trait Process {
    fn process(&self, path: &Path) -> Result<Vec<u8>>;
    fn extension(&self) -> &str;
}
