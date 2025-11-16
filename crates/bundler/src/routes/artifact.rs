use std::path::{Component, PathBuf};

use rocket::{fs::NamedFile, http::Status, tokio};
use uuid::Uuid;

use crate::routes::artifacts_dir;

fn check_is_empty(path: &PathBuf) -> bool {
    path.read_dir()
        .map_or(false, |mut dir| dir.next().is_none())
}

#[get("/artifact?<uuid>&<filepath..>")]
pub async fn artifact(uuid: String, filepath: String) -> Result<NamedFile, Status> {
    let base_path = artifacts_dir().map_err(|_| Status::InternalServerError)?;
    if Uuid::parse_str(&uuid).is_err() || filepath.is_empty() {
        return Err(Status::BadRequest);
    }
    let artifact_path = base_path.join(&uuid);
    let path = PathBuf::from(filepath);
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(Status::Forbidden);
    }

    let path = artifact_path.join(path);
    let file = NamedFile::open(&path).await.map_err(|_| Status::NotFound)?;
    if let Err(_) = tokio::fs::remove_file(&path).await {
        return Err(Status::InternalServerError);
    }

    if check_is_empty(&artifact_path) {
        if let Err(_) = tokio::fs::remove_dir_all(&artifact_path).await {
            return Err(Status::InternalServerError);
        }
    }

    Ok(file)
}
