use std::path::Path;

use anyhow::Result;
use image::{DynamicImage, ImageFormat};

use system::platform::Platform;

pub struct Icon {
    image: DynamicImage,
    format: ImageFormat,
}

impl Icon {
    pub fn from_bytes(target: &Platform, bytes: &[u8]) -> Option<Self> {
        let mut image = image::load_from_memory(bytes).ok()?;
        let ((width, height), format) = match target {
            Platform::Ctr => ((48, 48), ImageFormat::Png),
            Platform::Hac => ((256, 256), ImageFormat::Jpeg),
            Platform::Cafe => ((128, 128), ImageFormat::Png),
        };
        image = image.thumbnail(width, height);
        Some(Self { image, format })
    }

    pub fn create(&self, path: &Path) -> Result<()> {
        self.image.save_with_format(path, self.format)?;
        Ok(())
    }
}
