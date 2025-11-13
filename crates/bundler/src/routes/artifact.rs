use rocket::{fs::NamedFile, http::Status};

use crate::routes::artifacts_dir;

#[get("/artifact?<uuid>&<filepath..>")]
pub async fn artifact(uuid: String, filepath: String) -> Result<NamedFile, Status> {
    let base_path = artifacts_dir().map_err(|_| Status::InternalServerError)?;
    let artifact_path = base_path.join(uuid).join(filepath);

    match NamedFile::open(&artifact_path).await {
        Ok(artifact) => Ok(artifact),
        Err(_) => Err(Status::NotFound),
    }
}
