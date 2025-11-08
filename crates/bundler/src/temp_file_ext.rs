use std::io::{Result, Write};

use rocket::async_trait;
use rocket::fs::{NamedFile, TempFile};
use rocket::tokio::io::AsyncReadExt;
use tempfile::NamedTempFile;

#[async_trait]
pub trait TempFileExt {
    async fn read_bytes(&self) -> Result<Vec<u8>>;
}

#[async_trait]
impl<'f> TempFileExt for TempFile<'f> {
    async fn read_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.open().await?.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }
}

#[async_trait]
pub trait NamedFileExt {
    async fn from_bytes(bytes: &[u8]) -> Result<NamedFile>;
}

#[async_trait]
impl NamedFileExt for NamedFile {
    async fn from_bytes(bytes: &[u8]) -> Result<NamedFile> {
        let mut file = NamedTempFile::new()?;
        file.write_all(bytes)?;
        file.flush()?;

        let path = file.path().to_owned();
        Ok(NamedFile::open(path).await?)
    }
}
