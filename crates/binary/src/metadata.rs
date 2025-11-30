use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub targets: Vec<String>,
}
