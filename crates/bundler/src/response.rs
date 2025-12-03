use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ArtifactResponse {
    files: Vec<String>,
    token: Uuid,
}

impl ArtifactResponse {
    pub fn new(token: Uuid) -> Self {
        Self {
            files: Vec::new(),
            token,
        }
    }

    pub fn add_file(&mut self, filepath: PathBuf) {
        let filepath = filepath.to_string_lossy().replace("\\", "/");
        self.files.push(filepath);
    }

    pub fn json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}
