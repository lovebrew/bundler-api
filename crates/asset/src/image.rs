use std::path::Path;
use std::process::Command;
use std::{io::Cursor, path::PathBuf};

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
    fn process(&self, path: &Path, file_name: &Path) -> Result<PathBuf> {
        let program = system::programs::get_binary("tex3ds");
        let output_path = path.join(file_name).with_extension("t3x");

        Command::new(program)
            .args(["-f", "rgba"])
            .arg(path.join(file_name))
            .arg("-o")
            .arg(&output_path)
            .output()?;

        std::fs::remove_file(path.join(file_name))?;
        Ok(output_path.to_owned())
    }
}
