use std::path::Path;

use rocket::{
    form::{Form, FromForm},
    fs::TempFile,
    futures::future::join_all,
    http::Status,
    tokio,
};
use uuid::Uuid;

use crate::{response::ArtifactResponse, routes::artifacts_dir, tempfile::TempFileExt};
use asset::{font::Font, image::Image, process::Process};

#[derive(FromForm)]
pub struct AssetUpload<'f> {
    files: Vec<TempFile<'f>>,
    paths: Vec<String>,
}

#[post("/convert", format = "multipart/form-data", data = "<form>")]
pub async fn convert(form: Form<AssetUpload<'_>>) -> Result<String, Status> {
    if form.files.is_empty() || form.paths.is_empty() {
        return Err(Status::BadRequest);
    }

    if form.files.len() != form.paths.len() {
        return Err(Status::BadRequest);
    }

    let base_dir = artifacts_dir().map_err(|_| Status::InternalServerError)?;

    let token = Uuid::new_v4();
    let directory = base_dir.join(token.to_string());
    if let Err(e) = tokio::fs::create_dir_all(&directory).await {
        error!("Could not generate directory: {e}");
        return Err(Status::InternalServerError);
    }

    let form_data = form.files.iter().zip(form.paths.iter());
    let tasks = form_data.map(|(file, path)| {
        let directory = directory.clone();
        async move {
            if file.len() == 0 {
                return None;
            }

            let filepath = Path::new(file.name()?);
            let bytes = file.read_bytes().await.ok()?;

            let file_dir = directory.join(path);
            tokio::fs::create_dir_all(&file_dir).await.ok()?;

            let output_path = file_dir.join(file.name()?);
            if let Err(e) = tokio::fs::write(&output_path, &bytes).await {
                error!("Could not write file '{filepath:?}': {e}");
                return None;
            }

            let asset: Box<dyn Process + Send> = if Image::is_valid(&bytes).is_ok() {
                Box::new(Image {})
            } else if Font::is_valid(&bytes).is_ok() {
                Box::new(Font {})
            } else {
                return None;
            };

            let result = asset.process(&file_dir, &filepath);
            if let Err(result) = result {
                println!("{result:?}");
            } else if let Ok(filepath) = result {
                let path = filepath.strip_prefix(directory).ok()?;
                return Some(path.to_owned());
            }
            None
        }
    });

    let results: Vec<_> = join_all(tasks).await.into_iter().flatten().collect();

    let mut response = ArtifactResponse::new(token);
    for filepath in results.clone() {
        response.add_file(filepath);
    }

    match results.len() {
        0 => Err(Status::BadRequest),
        _ => response.json().map_err(|_| Status::InternalServerError),
    }
}
