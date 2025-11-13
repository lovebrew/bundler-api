use std::path::PathBuf;

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ArtifactResponse {
    files: Vec<PathBuf>,
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
        self.files.push(filepath);
    }

    pub fn json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
