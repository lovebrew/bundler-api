use std::io::Read;
use std::path::PathBuf;
use std::{collections::HashMap, io::Cursor};

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use octocrab::Octocrab;
use octocrab::models::repos::Asset;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

pub const CACHE_FILENAME: &str = ".cache";

#[derive(Serialize, Deserialize, Default)]
pub struct AssetCache {
    downloaded_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

const REPOSITORY_OWNER: &str = "lovebrew";
pub const RESOURCES_DIRECTORY: &str = "resources";
const REPOSITORIES: [(&str, Option<&str>); 2] = [("bundler", None), ("lovepotion", Some(".elf"))];

pub struct GitHubService {
    crab: Octocrab,
    client: Client,
    cache: HashMap<String, AssetCache>,
}

impl GitHubService {
    pub async fn new() -> Result<Self> {
        let crab = Octocrab::builder().build()?;
        let client = Client::new();
        let cache = HashMap::new();

        Ok(Self {
            crab,
            client,
            cache,
        })
    }

    async fn load_cache() -> Result<HashMap<String, AssetCache>> {
        if tokio::fs::try_exists(CACHE_FILENAME).await? {
            let contents = tokio::fs::read_to_string(CACHE_FILENAME).await?;
            let cache = serde_json::from_str(&contents)?;
            return Ok(cache);
        }
        Ok(HashMap::new())
    }

    async fn save_cache(&self) -> Result<()> {
        let contents = serde_json::to_string_pretty(&self.cache)?;
        tokio::fs::write(CACHE_FILENAME, contents).await?;
        Ok(())
    }

    fn is_asset_up_to_date(&self, asset_name: &str, updated_at: DateTime<Utc>) -> bool {
        if let Some(cache) = self.cache.get(asset_name) {
            return cache.downloaded_at > updated_at;
        }
        false
    }

    fn update_asset_cache(
        &mut self,
        name: String,
        downloaded_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) {
        self.cache.insert(
            name,
            AssetCache {
                downloaded_at,
                updated_at,
            },
        );
    }

    pub async fn sync() -> Result<()> {
        info!("Syncing GitHub resources...");
        let mut service = Self::new().await?;
        service.cache = Self::load_cache().await?;

        for &(repo, filter) in REPOSITORIES.iter() {
            let assets = service.get_release_assets(repo).await?;

            for asset in assets {
                if service.is_asset_up_to_date(&asset.name, asset.updated_at) {
                    info!("Skipping {} as it is already up-to-date.", asset.name);
                    continue;
                }

                let bytes = service.download_asset(asset.browser_download_url).await?;
                service.extract_zip(&asset.name, &bytes, filter).await?;
                info!("Downloaded and extracted asset: {}", asset.name);
                service.update_asset_cache(asset.name, Utc::now(), asset.updated_at);
            }
        }
        service.save_cache().await?;
        info!("GitHub assets sync completed successfully.");
        Ok(())
    }

    async fn get_release_assets(&self, repo: &str) -> Result<Vec<Asset>> {
        info!("Fetching latest release assets for repository: {repo}");
        let release = self
            .crab
            .repos(REPOSITORY_OWNER, repo)
            .releases()
            .get_latest()
            .await?;
        Ok(release.assets)
    }

    async fn download_asset(&self, url: Url) -> Result<Vec<u8>> {
        info!("Downloading asset from URL: {url}");
        let response = self.client.get(url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    async fn extract_zip(&self, name: &str, data: &[u8], filter: Option<&str>) -> Result<()> {
        info!("Extracting ZIP asset: {name}");
        let mut archive = ZipArchive::new(Cursor::new(data))?;
        let resources_path = PathBuf::from(RESOURCES_DIRECTORY);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            if !file.is_file() {
                continue;
            }

            let outpath = if let Some(filter) = filter {
                if !file.name().ends_with(filter) {
                    continue; // Skip files that do not match filter
                }

                let folder = if name.contains("3DS") {
                    "ctr"
                } else if name.contains("Switch") {
                    "hac"
                } else {
                    "cafe"
                };

                resources_path.join(folder).join(file.name())
            } else {
                resources_path.join(file.name())
            };

            if let Some(parent) = outpath.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let mut buffer = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut buffer)?;

            tokio::fs::write(outpath, &buffer).await?;
        }
        info!("Extracted {name} to {resources_path:#?}");
        Ok(())
    }
}
