use std::{path::Path, sync::Arc};

use axum::{extract::State, http::StatusCode};
use tokio::fs;
use tracing::{event, Level};

use crate::AppState;

pub async fn remove(
    State(app_state): State<Arc<AppState>>,
    filename: String,
) -> Result<StatusCode, String> {
    let path = Path::new(&app_state.save_dir).join(&filename);
    if !path.exists() {
        return Err(String::from("File does not exist"));
    }
    fs::remove_file(&path)
        .await
        .map_err(|e| format!("An error occurred while creating the file: {}", e))?;
    event!(
        Level::INFO,
        "File removed by API: {}",
        path.display().to_string()
    );

    Ok(StatusCode::OK)
}
