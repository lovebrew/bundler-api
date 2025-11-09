use std::fmt::Display;
use std::io::Write;
use std::sync::Arc;

use anyhow::{Result, bail};
use chrono::Local;
use rocket::tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct Tracefile {
    inner: Arc<RwLock<Vec<u8>>>,
}

enum Level {
    Info,
    Error,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Info => write!(f, "INFO"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}

impl Tracefile {
    async fn log(&self, level: Level, message: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let mut data = self.inner.write().await;
        writeln!(data, "[{timestamp} {level: >}] {message}")?;
        Ok(())
    }

    pub async fn info(&self, message: &str) {
        let _ = self.log(Level::Info, message).await;
    }

    pub async fn error(&self, message: &str) {
        let _ = self.log(Level::Error, message).await;
    }

    pub async fn bytes(&self) -> Result<Vec<u8>> {
        let data = self.inner.read().await;
        if data.is_empty() {
            bail!("No log data");
        }
        Ok(data.clone())
    }
}
