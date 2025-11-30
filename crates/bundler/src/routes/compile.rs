use std::str::FromStr;

use anyhow::Result;
use asset::icon::Icon;
use binary::{cafe::Cafe, compile::Compile, ctr::Ctr, hac::Hac, metadata::Metadata};
use rocket::{
    form::{Form, FromForm},
    fs::TempFile,
    futures::future::join_all,
    http::Status,
    tokio,
};
use system::{platform::Platform, resources};
use uuid::Uuid;

use crate::{response::ArtifactResponse, routes::artifacts_dir, tempfile::TempFileExt};

#[derive(FromForm, Debug)]
pub struct CompileRequest<'f> {
    pub config: String,
    pub icon: Option<TempFile<'f>>,
}

#[post("/compile", data = "<form>")]
pub async fn compile(form: Form<CompileRequest<'_>>) -> Result<String, Status> {
    let mut metadata = match serde_json::from_str::<Metadata>(&form.config) {
        Ok(metadata) => metadata,
        Err(_) => return Err(Status::BadRequest),
    };
    metadata.targets.dedup();

    let base_dir = artifacts_dir().map_err(|_| Status::InternalServerError)?;

    let token = Uuid::new_v4();
    let directory = base_dir.join(token.to_string());
    if let Err(e) = tokio::fs::create_dir_all(&directory).await {
        error!("Could not generate directory: {e}");
        return Err(Status::InternalServerError);
    }

    let icon_bytes = match &form.icon {
        Some(icon) if icon.len() > 0 => icon.read_bytes().await,
        _ => tokio::fs::read(resources::fetch_icon()).await,
    }
    .map_err(|_| Status::InternalServerError)?;

    let tasks = metadata.targets.clone().into_iter().map(|target| {
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
            let _ = Icon::from_bytes(&platform, &icon_bytes)?.create(&icon_path);
            let binary: Box<dyn Compile + Send> = match platform {
                Platform::Ctr => Box::new(Ctr {}),
                Platform::Hac => Box::new(Hac {}),
                Platform::Cafe => Box::new(Cafe {}),
            };
            let result = binary.compile(&target_path, &metadata, &icon_path);
            tokio::fs::remove_file(icon_path).await.ok()?;

            if let Err(result) = result {
                println!("{result:?}");
            } else if let Ok(path) = result {
                let path = path.strip_prefix(directory).ok()?;
                return Some(path.to_owned());
            }
            None
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
        0 => Err(Status::BadRequest),
        _ => response.json().map_err(|_| Status::InternalServerError),
    }
}
