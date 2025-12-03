use std::io::Result;

use rocket::async_trait;
use rocket::fs::TempFile;
use rocket::tokio::io::AsyncReadExt;

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
