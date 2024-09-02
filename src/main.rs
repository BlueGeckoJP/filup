mod api;
mod route;

use api::*;
use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};
use clap::Parser;
use once_cell::sync::{Lazy, OnceCell};
use route::*;
use std::{error::Error, fs, path::Path};
use tera::Tera;
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::{event, instrument, Level};

pub static TEMPLATES: Templates = Templates { t: OnceCell::new() };
pub static SAVE_DIR: Lazy<String> = Lazy::new(|| String::from("./files"));

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

pub struct Templates {
    t: OnceCell<Mutex<Tera>>,
}

impl Templates {
    async fn update(&self) -> Result<(), Box<dyn Error>> {
        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => return Err(Box::new(e)),
        };
        {
            if self.t.get().is_none() {
                self.t.set(Mutex::new(tera)).unwrap();
            } else {
                let mut t = self.t.get().unwrap().lock().await;
                *t = tera;
            }
        }
        Ok(())
    }
}

async fn check_dir_exists() -> Result<(), Box<dyn Error>> {
    let save_dir = SAVE_DIR.clone();
    if !Path::new(&save_dir).exists() {
        if let Err(e) = fs::create_dir(&save_dir) {
            return Err(Box::new(e));
        }
        event!(Level::INFO, "mkdir SAVE_DIR ({})", save_dir);
    }
    event!(Level::INFO, "check_dir_exists() completed!");
    Ok(())
}

#[instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).pretty().init();

    TEMPLATES.update().await.unwrap();

    let args = Args::parse();
    event!(Level::INFO, "The following args were received: {:?}", args);

    check_dir_exists().await.unwrap();

    let assets_service = get_service(ServeDir::new("assets")).handle_error(|e| async move {
        (StatusCode::NOT_FOUND, format!("asset not found: {}", e))
    });
    let files_service = get_service(ServeDir::new("files")).handle_error(|e| async move {
        (StatusCode::NOT_FOUND, format!("files not found: {}", e))
    });
    let app = Router::new()
        .nest_service("/assets", assets_service)
        .nest_service("/files", files_service)
        .route("/", get(route_root::root))
        .route("/api/upload", post(api_upload::upload))
        .route("/api/remove", post(api_remove::remove))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 10)); // 10 GiB (tabun)

    event!(Level::INFO, "Listening on 0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
