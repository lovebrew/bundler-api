use rocket::{
    Request, Response,
    fairing::{Fairing, Info, Kind},
    http::{Header, Method, Status},
};
use std::{collections::HashSet, sync::LazyLock};

use log::error;

pub struct Cors;

static ALLOWED_ORIGINS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set: HashSet<&'static str> =
        ["https://lovebrew.github.io", "https://bundle.lovebrew.org"]
            .into_iter()
            .collect();

    #[cfg(debug_assertions)]
    {
        set.insert("http://localhost:3000");
    }
    set
});

static CORS_PATHS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["/convert", "/compile", "/artifact"]));

fn set_cors_headers(
    response: &mut Response<'_>,
    origin: impl ToString,
    methods: impl ToString,
    headers: impl ToString,
) {
    response.set_header(Header::new(
        "Access-Control-Allow-Origin",
        origin.to_string(),
    ));
    response.set_header(Header::new(
        "Access-Control-Allow-Methods",
        methods.to_string(),
    ));
    response.set_header(Header::new(
        "Access-Control-Allow-Headers",
        headers.to_string(),
    ));
    response.set_header(Header::new("Access-Control-Max-Age", "86400"));
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Selective CORS Headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let headers = request.headers();
        let req_headers = headers
            .get_one("Access-Control-Request-Headers")
            .unwrap_or("Content-Type, Authorization");

        let path = request.uri().path();

        if path == "/health" {
            set_cors_headers(response, "*", "GET, OPTIONS", req_headers);
            return;
        }

        if CORS_PATHS.contains(path.as_str()) {
            let origin = headers.get_one("Origin");
            if let Some(origin) = origin {
                if ALLOWED_ORIGINS.contains(&origin) {
                    set_cors_headers(response, origin, "POST, OPTIONS", req_headers);
                } else {
                    error!("Unauthorized CORS origin: {origin}!");
                }
            }
        }

        let method = request.method();

        if method == Method::Options {
            if response.status() == Status::NotFound {
                response.set_status(Status::NoContent);
            }
            if !response.headers().contains("Access-Control-Allow-Origin") {
                set_cors_headers(response, "*", "OPTIONS", req_headers);
            }
        }
    }
}
