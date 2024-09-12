use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use tokio::fs;
use tracing::{event, Level};

use crate::{path_check, AppState};

pub async fn remove(
    State(app_state): State<Arc<AppState>>,
    filename: String,
) -> Result<StatusCode, String> {
    fs::remove_file(
        path_check(&app_state.save_dir, &filename)
            .await
            .map_err(|e| e.to_string())?,
    )
    .await
    .map_err(|e| format!("An error occurred while creating the file: {}", e))?;
    event!(Level::INFO, "File removed by API: {}", &filename);

    Ok(StatusCode::OK)
}
