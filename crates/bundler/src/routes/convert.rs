use std::path::Path;

use rocket::{
    form::{Form, FromForm},
    fs::{NamedFile, TempFile},
    futures::future::join_all,
    http::Status,
    tokio,
};
use tempfile::tempdir;

use crate::temp_file_ext::{NamedFileExt, TempFileExt};
use crate::zipfile::ZipFile;
use asset::{font::Font, image::Image, process::Process};

#[derive(FromForm)]
pub struct AssetUpload<'f> {
    files: Vec<TempFile<'f>>,
    paths: Vec<String>,
}

#[post("/convert", format = "multipart/form-data", data = "<form>")]
pub async fn convert(form: Form<AssetUpload<'_>>) -> Result<(Status, NamedFile), Status> {
    if form.files.is_empty() || form.paths.is_empty() {
        return Err(Status::BadRequest);
    }

    if form.files.len() != form.paths.len() {
        return Err(Status::BadRequest);
    }

    let temp_dir = tempdir().map_err(|_| Status::InternalServerError)?;

    let form_data = form.files.iter().zip(form.paths.iter());
    let tasks = form_data.map(|(file, path)| {
        let temp_path = temp_dir.path().to_owned();
        async move {
            if file.len() == 0 {
                return None;
            }

            if Path::new(path).has_root() {
                return None;
            }

            let file_path = file.name()?;
            let bytes = file.read_bytes().await.ok()?;
            let output_path = std::path::absolute(temp_path.join(path).join(file_path)).ok()?;
            if let Some(parent) = output_path.parent() {
                tokio::fs::create_dir_all(parent).await.ok()?;
            }
            tokio::fs::write(&output_path, &bytes).await.ok()?;

            let asset: Box<dyn Process + Send> = if Image::is_valid(&bytes).is_ok() {
                Box::new(Image {})
            } else if Font::is_valid(&bytes).is_ok() {
                Box::new(Font {})
            } else {
                return None;
            };

            let bytes = asset.process(&output_path).ok()?;
            let filepath = output_path.strip_prefix(temp_path).ok()?;
            let extension = asset.extension();
            Some((filepath.with_extension(extension), bytes))
        }
    });

    let results: Vec<_> = join_all(tasks).await.into_iter().flatten().collect();
    if results.is_empty() {
        return Err(Status::BadRequest);
    }

    let mut zip = ZipFile::new();
    for (output_path, bytes) in results {
        let _ = zip.add_file(&output_path, &bytes);
    }

    let file_count = zip.file_count();
    let zip_content = zip.finish().map_err(|_| Status::InternalServerError)?;

    let mut status = Status::Ok;
    if file_count < form.files.len() {
        status = Status::PartialContent;
    }

    let result = NamedFile::from_bytes(&zip_content)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok((status, result))
}
