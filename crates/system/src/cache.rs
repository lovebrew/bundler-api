use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const CACHE_FILENAME: &str = ".cache";

#[derive(Serialize, Deserialize, Default)]
struct AssetTimestamp {
    downloaded_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AssetCache {
    cache: HashMap<String, AssetTimestamp>,
}

impl AssetCache {
    pub fn load() -> Result<AssetCache> {
        if !Path::new(CACHE_FILENAME).exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(CACHE_FILENAME)?;
        let cache: HashMap<String, AssetTimestamp> = serde_json::from_str(&contents)?;
        Ok(Self { cache })
    }

    pub fn update(&mut self, name: &str, timestamp: DateTime<Utc>) -> Result<()> {
        self.cache.insert(
            name.to_string(),
            AssetTimestamp {
                downloaded_at: Utc::now(),
                updated_at: timestamp,
            },
        );
        let contents = serde_json::to_string_pretty(&self.cache)?;
        std::fs::write(CACHE_FILENAME, contents)?;
        Ok(())
    }

    pub fn is_up_to_date(&self, name: &str, timestamp: DateTime<Utc>) -> bool {
        if let Some(cache) = self.cache.get(name) {
            cache.updated_at >= timestamp
        } else {
            false
        }
    }
}
