use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct FileDetails {
    size: String,
    creation_time: String,
}

pub async fn details(filename: String) -> Json<FileDetails> {}
