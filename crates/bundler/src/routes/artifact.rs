use std::path::PathBuf;

use rocket::{fs::NamedFile, http::Status};
use uuid::Uuid;

#[get("/artifact/<uuid>/<filename..>")]
pub fn artifact(uuid: String, filename: PathBuf) -> Result<(Status, NamedFile), Status> {
    todo!()
}
