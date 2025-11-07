use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use asset::icon::Icon;
use binary::{compile::Compile, ctr::Ctr, metadata::Metadata};
use rocket::{
    form::{Form, FromForm},
    fs::{NamedFile, TempFile},
    futures::future::join_all,
    http::Status,
    tokio,
};
use system::{platform::Platform, resources, resources::Resource};
use tempfile::tempdir;

use crate::{
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
pub async fn compile(form: Form<CompileRequest<'_>>) -> Result<(Status, NamedFile), Status> {
    let mut form = form.into_inner();
    form.target.dedup();
    if form.target.is_empty() {
        return Err(Status::BadRequest);
    }

    let temp_dir = tempdir().map_err(|_| Status::InternalServerError)?;

    let metadata = Metadata {
        title: form.title,
        author: form.author,
        version: form.version,
        description: form.description,
    };

    let icon_data = &form.icon.take();
    let tasks = form.target.clone().into_iter().map(|target| {
        let temp_path = temp_dir.path().to_owned();
        let metadata = metadata.clone();
        async move {
            let platform = Platform::from_str(&target).ok()?;
            let target_path = temp_path.join(target);
            let icon_path = target_path.join("icon.bin");

            let icon_bytes = if icon_data.is_none() {
                let path = resources::fetch(&platform, Resource::DefaultIcon);
                tokio::fs::read(path).await.ok()?
            } else {
                let file = icon_data.as_ref()?;
                file.read_bytes().await.ok()?
            };
            Icon::from_bytes(&platform, &icon_bytes)?
                .create(&icon_path)
                .ok()?;

            let binary = match platform {
                Platform::Ctr => Box::new(Ctr {}),
                _ => return None,
            };
            binary.compile(&target_path, &metadata, &icon_path).ok()
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
    if file_count < form.target.len() {
        status = Status::PartialContent;
    }

    let result = NamedFile::from_bytes(&zip_content)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok((status, result))
}
