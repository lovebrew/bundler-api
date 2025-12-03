use rocket::{Build, Rocket};

use crate::cors::Cors;
use crate::routes::{artifact::artifact, compile::compile, convert::convert, health::health};

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![artifact, compile, convert, health])
        .attach(Cors)
}
