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
use route::*;
use std::{
    collections::HashMap,
    error::Error,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tera::Tera;
use tokio::{
    net::TcpListener,
    sync::{
        broadcast::{Receiver, Sender},
        Mutex,
    },
};
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::{event, instrument, span, Level};

pub type ProgChannels = Mutex<HashMap<String, (Sender<usize>, Receiver<usize>)>>;

struct AppState {
    save_dir: String,
    templates: Mutex<Tera>,
    prog_channels: ProgChannels,
}

impl AppState {
    async fn templates_new() -> Result<Tera, Box<dyn Error>> {
        let tera = Tera::new("templates/*.html")?;
        Ok(tera)
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

async fn check_dir_exists(save_dir: &String) -> Result<(), Box<dyn Error>> {
    let _span = span!(Level::INFO, "check_dir_exists").entered();

    if !Path::new(&save_dir).exists() {
        if let Err(e) = fs::create_dir(save_dir) {
            return Err(Box::new(e));
        }
        event!(
            Level::INFO,
            "SAVE_DIR ({}) was not found!, Created a new one",
            save_dir
        );
    }
    event!(Level::INFO, "Directory exists check complete!");
    Ok(())
}

pub async fn path_check<S: AsRef<OsStr> + ?Sized>(
    save_dir: &String,
    s: &S,
) -> Result<PathBuf, Box<dyn Error>> {
    let _span = span!(Level::INFO, "path_check").entered();

    let original_path = Path::new(s);
    event!(Level::INFO, "In - {}", &original_path.to_string_lossy());

    let filename = match original_path.file_name() {
        Some(filename) => filename,
        None => {
            event!(Level::WARN, "Error - {}", &original_path.to_string_lossy());
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Could not retrieve file name: {}",
                    &original_path.to_string_lossy()
                ),
            )));
        }
    };

    let path = Path::new(save_dir).join(filename);
    event!(Level::INFO, "Out - {}", &path.to_string_lossy());

    Ok(path)
}

#[instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let app_state = Arc::new(AppState {
        save_dir: String::from("./files"),
        templates: Mutex::new(AppState::templates_new().await.unwrap()),
        prog_channels: Mutex::new(HashMap::new()),
    });

    let args = Args::parse();
    event!(Level::INFO, "The following args were received: {:?}", args);

    check_dir_exists(&app_state.save_dir).await.unwrap();

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
        .route("/api/progress", get(api_progress::progress))
        .route("/api/details", post(api_details::details))
        .route("/api/hash", post(api_hash::hash))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 100)) // 100 GiB (tabun)
        .with_state(app_state);

    event!(Level::INFO, "Listening on 0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
