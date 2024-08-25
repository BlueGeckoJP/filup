use std::path::Path;

use axum::http::StatusCode;
use tokio::fs;
use tracing::{event, Level};

use crate::SAVE_DIR;

pub async fn remove(filename: String) -> StatusCode {
    let path = Path::new(&SAVE_DIR.clone()).join(&filename);
    if !path.exists() {
        return StatusCode::BAD_REQUEST;
    }
    fs::remove_file(&path).await.unwrap();
    event!(
        Level::INFO,
        "File removed by API: {}",
        path.display().to_string()
    );

    StatusCode::OK
}
