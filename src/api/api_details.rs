use std::{os::unix::fs::MetadataExt, sync::Arc};

use axum::{extract::State, Json};
use chrono::{DateTime, Local};
use serde::Serialize;
use tokio::fs;

use crate::AppState;

#[derive(Serialize, Default)]
pub struct FileDetails {
    size: String,
    creation_time: String,
}

pub async fn details(
    State(app_state): State<Arc<AppState>>,
    filename: String,
) -> Result<Json<FileDetails>, String> {
    if filename.is_empty() {
        return Err("The filename (body) is empty".to_string());
    }

    let metadata = match fs::metadata(format!("{}/{}", &app_state.save_dir, filename)).await {
        Ok(metadata) => metadata,
        Err(e) => return Err(format!("File does not exist: {}", e)),
    };

    let kib = 1024f64;
    let mib = kib * 1024f64;
    let gib = mib * 1024f64;
    let human_readable_size = match metadata.size() as f64 {
        x if x >= gib => format!("{:.2} GiB", (x / gib)),
        x if x >= mib => format!("{:.2} MiB", (x / mib)),
        x if x >= kib => format!("{:.2} KiB", (x / kib)),
        x => format!("{} B", x),
    };

    let created_time = DateTime::<Local>::from(
        metadata
            .created()
            .map_err(|e| format!("Could not retrieve file creation date/time: {}", e))?,
    );
    let readable_created_time = created_time.format("%Y/%m/%d %H:%M:%S %Z").to_string();

    Ok(Json(FileDetails {
        size: human_readable_size,
        creation_time: readable_created_time,
    }))
}
