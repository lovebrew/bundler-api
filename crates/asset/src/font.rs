use std::{path::Path, process::Command};

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
    fn process(&self, path: &Path) -> Result<Vec<u8>> {
        let program = system::programs::get_binary("mkbcfnt");

        let output_name = path.with_extension("bcfnt");
        let output = Command::new(program)
            .arg(path)
            .arg("-o")
            .arg(&output_name)
            .output();

        let bytes = match output {
            Ok(_) => std::fs::read(&output_name)?,
            Err(_) => bail!("Failed to convert {path:?}"),
        };
        Ok(bytes)
    }
}
