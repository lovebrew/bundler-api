use std::path::{Component, PathBuf};

use rocket::{fs::NamedFile, http::Status};
use uuid::Uuid;

use crate::routes::artifacts_dir;

#[get("/artifact?<uuid>&<filepath..>")]
pub async fn artifact(uuid: String, filepath: String) -> Result<NamedFile, Status> {
    let base_path = artifacts_dir().map_err(|_| Status::InternalServerError)?;
    if Uuid::parse_str(&uuid).is_err() || filepath.is_empty() {
        return Err(Status::BadRequest);
    }

    let path = PathBuf::from(filepath);
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(Status::Forbidden);
    }

    let path = base_path.join(&uuid).join(path);
    let file = NamedFile::open(path).await.map_err(|_| Status::NotFound)?;

    // TODO: check uuid directory is empty, recursively and delete it if it is

    Ok(file)
}
