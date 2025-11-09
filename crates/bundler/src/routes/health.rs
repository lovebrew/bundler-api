use rocket::tokio::{self, sync::OnceCell};
use system::cache::CACHE_FILENAME;

static DATA: OnceCell<String> = OnceCell::const_new();

async fn load_cache() -> String {
    match tokio::fs::read_to_string(CACHE_FILENAME).await {
        Ok(content) => content,
        Err(_) => String::from("OK"),
    }
}

#[get("/health")]
pub async fn health() -> String {
    DATA.get_or_init(load_cache).await.to_string()
}
