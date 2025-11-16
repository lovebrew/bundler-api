#[macro_use]
extern crate rocket;

mod cors;
mod logger;
mod response;
mod routes;
pub mod server;
mod tempfile;

use system::programs;

use anyhow::Result;
use log::error;

use server::rocket;

const CONFIG: &str = include_str!("../log4rs.yml");

#[rocket::main]
async fn main() -> Result<()> {
    let config = serde_yaml::from_str(CONFIG)?;
    log4rs::init_raw_config(config)?;

    if let Err(error) = programs::check_environment() {
        error!("{error}");
        std::process::exit(1);
    }

    system::downloads::sync().await?;
    rocket().launch().await?;

    Ok(())
}
