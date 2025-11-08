use std::io::{Cursor, Write};
use std::path::Path;

use anyhow::{Result, bail};
use zip::{ZipWriter, write::SimpleFileOptions};

pub struct ZipFile {
    writer: ZipWriter<Cursor<Vec<u8>>>,
    options: SimpleFileOptions,
    file_count: usize,
}

impl ZipFile {
    pub fn new() -> Self {
        let cursor = Cursor::new(Vec::new());
        let writer = ZipWriter::new(cursor);
        let options = SimpleFileOptions::default();

        Self {
            writer,
            options,
            file_count: 0,
        }
    }

    pub fn add_file(&mut self, path: &Path, bytes: &[u8]) -> Result<()> {
        if bytes.is_empty() {
            bail!("Cannot add empty file.");
        }
        self.writer.start_file_from_path(path, self.options)?;
        self.writer.write_all(bytes)?;
        self.file_count += 1;
        Ok(())
    }

    pub fn file_count(&self) -> usize {
        self.file_count
    }

    pub fn finish(self) -> Result<Vec<u8>> {
        let cursor = self.writer.finish()?;
        Ok(cursor.into_inner())
    }
}
