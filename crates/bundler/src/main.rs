#[macro_use]
extern crate rocket;

mod cors;
mod logger;
mod response;
mod routes;
mod temp_file_ext;
mod zipfile;

use routes::{artifact::artifact, compile::compile, convert::convert, health::health};

use system::programs;

use anyhow::Result;
use log::error;

use crate::cors::Cors;

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

    let rocket = rocket::build();
    info!("Running with profile: {}", rocket.figment().profile());

    rocket
        .mount("/", routes![health, convert, compile, artifact])
        .attach(Cors)
        .launch()
        .await?;

    Ok(())
}
