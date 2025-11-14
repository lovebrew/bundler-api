use std::str::FromStr;

use anyhow::Result;
use asset::icon::Icon;
use binary::{cafe::Cafe, compile::Compile, ctr::Ctr, hac::Hac, metadata::Metadata};
use rocket::{
    form::{Form, FromForm},
    fs::{NamedFile, TempFile},
    futures::future::join_all,
    http::Status,
    tokio,
};
use system::{platform::Platform, resources::Resource};
use tempfile::tempdir;
use uuid::Uuid;

use crate::{
    response::ArtifactResponse,
    routes::artifacts_dir,
    temp_file_ext::{NamedFileExt, TempFileExt},
    zipfile::ZipFile,
};

#[derive(FromForm, Debug)]
pub struct CompileRequest<'f> {
    pub title: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub target: Vec<String>,
    pub icon: Option<TempFile<'f>>,
}

#[post("/compile", data = "<form>")]
pub async fn compile(form: Form<CompileRequest<'_>>) -> Result<String, Status> {
    let mut form = form.into_inner();
    form.target.dedup();
    if form.target.is_empty() {
        return Err(Status::BadRequest);
    }

    let base_dir = artifacts_dir().map_err(|_| Status::InternalServerError)?;

    let token = Uuid::new_v4();
    let directory = base_dir.join(token.to_string());
    if let Err(e) = tokio::fs::create_dir_all(&directory).await {
        error!("Could not generate directory: {e}");
        return Err(Status::InternalServerError);
    }

    let metadata = Metadata {
        title: form.title,
        author: form.author,
        version: form.version,
        description: form.description,
    };

    let icon_bytes = match form.icon {
        Some(icon) if icon.len() > 0 => icon.read_bytes().await.ok(),
        _ => None,
    };

    let tasks = form.target.clone().into_iter().map(|target| {
        let metadata = metadata.clone();
        let icon_bytes = icon_bytes.clone();
        let directory = directory.clone();
        async move {
            let platform = Platform::from_str(&target).ok()?;
            let target_path = directory.join(target);
            if !target_path.exists() {
                tokio::fs::create_dir_all(&target_path).await.ok()?;
            }
            let icon_path = target_path.join("icon.bin");
            let icon_data = match icon_bytes {
                Some(bytes) => bytes,
                None => {
                    let path = system::resources::fetch(&platform, Resource::DefaultIcon);
                    let bytes = tokio::fs::read(path).await.ok()?;
                    bytes
                }
            };
            let _ = Icon::from_bytes(&platform, &icon_data)?.create(&icon_path);
            let binary: Box<dyn Compile + Send> = match platform {
                Platform::Ctr => Box::new(Ctr {}),
                Platform::Hac => Box::new(Hac {}),
                Platform::Cafe => Box::new(Cafe {}),
            };
            binary.compile(&target_path, &metadata, &icon_path).ok()
        }
    });

    let results: Vec<_> = join_all(tasks).await.into_iter().flatten().collect();
    if results.is_empty() {
        return Err(Status::BadRequest);
    }

    let mut response = ArtifactResponse::new(token);
    for filepath in results.clone() {
        response.add_file(filepath);
    }

    match results.len() {
        0 => return Err(Status::BadRequest),
        _ => response.json().map_err(|_| Status::InternalServerError),
    }
}
