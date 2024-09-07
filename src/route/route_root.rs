use std::sync::Arc;

use axum::{extract::State, response::Html};
use serde::Serialize;
use tera::Context;
use tokio::fs;

use crate::AppState;

#[derive(Debug, Serialize)]
struct FileItem {
    filename: String,
    path: String,
}

pub async fn root(State(app_state): State<Arc<AppState>>) -> Result<Html<String>, String> {
    let mut entries = fs::read_dir("./files")
        .await
        .map_err(|e| format!("Could not retrieve file list: {}", e))?;
    let mut file_list_vec: Vec<FileItem> = vec![];
    while let Some(file) = entries
        .next_entry()
        .await
        .map_err(|e| format!("Error retrieving next file entry: {}", e))?
    {
        let filename = file.file_name().to_string_lossy().to_string();
        let path = format!("/files/{}", filename);
        let item = FileItem { filename, path };
        file_list_vec.push(item);
    }

    let mut context = Context::new();
    context.insert("file_list", &file_list_vec);

    let view = app_state
        .templates
        .lock()
        .await
        .render("root.html", &context)
        .expect("Failed to load TEMPLATES (root.html)");
    Ok(Html(view))
}
