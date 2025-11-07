use std::io::Cursor;
use std::path::Path;
use std::process::Command;

use anyhow::{Result, bail};
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader};

use crate::process::Process;

pub struct Image;

impl Image {
    fn validate(image: &DynamicImage) -> bool {
        let (width, height) = image.dimensions();
        (3..=1024).contains(&width) && (3..=1024).contains(&height)
    }

    pub fn is_valid(bytes: &[u8]) -> Result<()> {
        let reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
        match reader.format() {
            Some(ImageFormat::Png | ImageFormat::Jpeg) => {
                let image = reader.decode()?;
                if Self::validate(&image) {
                    return Ok(());
                }
                bail!("Invalid image.")
            }
            _ => bail!("Failed to read image."),
        }
    }
}

impl Process for Image {
    fn process(&self, path: &Path) -> Result<Vec<u8>> {
        let program = system::programs::get_binary("tex3ds");

        let output_name = path.with_extension("t3x");
        let output = Command::new(program)
            .args(["-f", "rgba"])
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
