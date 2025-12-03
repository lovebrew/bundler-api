use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::Result;
use log::info;
use reqwest::Client;
use tempfile::TempDir;
use zip::ZipArchive;

use crate::cache::AssetCache;

struct RepoConfig<'a> {
    pub name: &'a str,
    pub filter: Option<&'a str>,
}

impl RepoConfig<'_> {
    fn subfolder_for(asset_name: &str) -> &str {
        if asset_name.contains("3DS") {
            "ctr"
        } else if asset_name.contains("Switch") {
            "hac"
        } else {
            "cafe"
        }
    }
}

pub const RESOURCES_DIRECTORY: &str = "resources";

const REPO_OWNER: &str = "lovebrew";
const REPOSITORIES: &[RepoConfig] = &[
    RepoConfig {
        name: "bundler-assets",
        filter: None,
    },
    RepoConfig {
        name: "lovepotion",
        filter: Some("lovepotion.elf"),
    },
];

async fn extract_files(file_path: &Path, filter: Option<&str>) -> Result<()> {
    info!("Extracting files from {file_path:?}");
    let file = File::open(file_path)?;
    let mut zip_file = ZipArchive::new(file)?;
    if filter.is_none() {
        zip_file.extract(RESOURCES_DIRECTORY)?;
    } else {
        let asset_name = file_path.to_string_lossy();
        let subfolder = RepoConfig::subfolder_for(&asset_name);
        if let Some(filter_name) = filter {
            let file_path = format!("{RESOURCES_DIRECTORY}/{subfolder}/{filter_name}");
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

    for repo_config in REPOSITORIES {
        info!("Fetching assets from {REPO_OWNER}/{}", repo_config.name);
        let repository = octocrab.repos(REPO_OWNER, repo_config.name);
        let releases = repository.releases().get_latest().await?;

        for asset in releases.assets {
            if !cache.is_up_to_date(&asset.name, asset.updated_at) {
                let response = client.get(asset.browser_download_url).send().await?;
                let bytes = response.bytes().await?;
                let file_path = directory.path().join(&asset.name);
                tokio::fs::write(&file_path, bytes).await?;

                extract_files(&file_path, repo_config.filter).await?;
                info!("Downloaded and extracted asset: {}", asset.name);
                cache.update(&asset.name, asset.updated_at)?;
            } else {
                info!("Asset {} is up to date.", asset.name);
            }
        }
    }
    info!("GitHub assets sync completed successfully.");
    Ok(())
}
