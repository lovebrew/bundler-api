use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::Result;
use log::info;
use reqwest::Client;
use tempfile::TempDir;
use zip::ZipArchive;

use crate::cache::AssetCache;

const REPOSITORIES: [(&str, Option<&str>); 2] =
    [("bundler", None), ("lovepotion", Some("lovepotion.elf"))];
pub const RESOURCES_DIRECTORY: &str = "resources";
const OWNER: &str = "lovebrew";

async fn extract_files(file_path: &Path, filter: Option<&str>) -> Result<()> {
    info!("Extracting files from {file_path:?}");
    let file = File::open(file_path)?;
    let mut zip_file = ZipArchive::new(file)?;
    if filter.is_none() {
        zip_file.extract(RESOURCES_DIRECTORY)?;
    } else {
        let zip_name = file_path.to_string_lossy();
        let sub_folder = if zip_name.contains("3DS") {
            "ctr"
        } else if zip_name.contains("Switch") {
            "hac"
        } else {
            "cafe"
        };
        if let Some(filter_name) = filter {
            let file_path = format!("{RESOURCES_DIRECTORY}/{sub_folder}/{filter_name}");
            let mut buf = Vec::new();
            zip_file.by_name(filter_name)?.read_to_end(&mut buf)?;
            tokio::fs::write(file_path, buf).await?;
        }
    }
    Ok(())
}

pub async fn sync() -> Result<()> {
    info!("Syncing GitHub resources...");
    let octocrab = octocrab::instance();
    let client = Client::new();
    let directory = TempDir::new()?;
    let mut cache = AssetCache::load()?;

    for (repo, filter) in REPOSITORIES {
        info!("Fetching assets from lovebrew/{repo}");
        let repository = octocrab.repos(OWNER, repo);
        let releases = repository.releases().get_latest().await?;
        for asset in releases.assets {
            let response = client.get(asset.browser_download_url).send().await?;
            let bytes = response.bytes().await?;
            let file_path = directory.path().join(&asset.name);
            tokio::fs::write(&file_path, bytes).await?;

            if !cache.is_up_to_date(&asset.name, asset.updated_at) {
                extract_files(&file_path, filter).await?;
                info!("Downloaded and extracted asset: {}", asset.name);
                cache.update(&asset.name, asset.updated_at)?;
            }
        }
    }
    info!("GitHub assets sync completed successfully.");
    Ok(())
}
