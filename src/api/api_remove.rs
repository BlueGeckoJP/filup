use std::{path::Path, sync::Arc};

use axum::{extract::State, http::StatusCode};
use tokio::fs;
use tracing::{event, Level};

use crate::AppState;

pub async fn remove(State(app_state): State<Arc<AppState>>, filename: String) -> StatusCode {
    let path = Path::new(&app_state.save_dir).join(&filename);
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
