use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Result, bail};
use ttf_parser::Face;

use crate::process::Process;

pub struct Font;

impl Font {
    pub fn is_valid(bytes: &[u8]) -> Result<()> {
        match Face::parse(bytes, 0) {
            Ok(_) => Ok(()),
            Err(_) => bail!("Invalid font."),
        }
    }
}

impl Process for Font {
    fn process(&self, path: &Path, file_name: &Path) -> Result<PathBuf> {
        let program = system::programs::get_binary("mkbcfnt");
        let output_path = path.join(file_name).with_extension("bcfnt");

        let output = Command::new(program)
            .arg(path.join(file_name))
            .arg("-o")
            .arg(&output_path)
            .output();

        let output_name = output_path.strip_prefix(path)?;

        match output {
            Ok(_) => Ok(output_name.to_owned()),
            Err(_) => bail!("Failed to convert {path:?}"),
        }
    }
}
