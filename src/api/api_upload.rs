use std::sync::Arc;

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use base64::{engine::general_purpose, Engine};
use tokio::{fs::File, io::AsyncWriteExt, sync::broadcast::Sender};
use tracing::{event, Level};

use crate::{path_check, AppState};

pub async fn upload(
    State(app_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<StatusCode, String> {
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| format!("Error while retrieving next field in multipart: {}", e))?
    {
        let filename = match field.file_name() {
            Some(name) => name.to_owned(),
            None => return Err(String::from("Could not get file name from field")),
        };

        let encoded_filename = general_purpose::URL_SAFE.encode(&filename).replace("=", "");
        let tx: Sender<usize>;
        {
            let hashmap = app_state.prog_channels.lock().await;
            let item = match hashmap.get(&encoded_filename) {
                Some(item) => item,
                None => return Err(String::from(
                    "There was no connection to ProgressAPI. Please connect to ProgressAPI first",
                )),
            };
            let original_tx = &item.0;
            tx = original_tx.clone();
        }
        event!(
            Level::INFO,
            "TX | Connected to Progress API: {}",
            encoded_filename
        );

        event!(Level::INFO, "Starting upload: {}", filename);

        let mut file = File::create(
            path_check(&app_state.save_dir, &filename)
                .await
                .map_err(|e| e.to_string())?,
        )
        .await
        .map_err(|e| format!("An error occurred while creating the file: {}", e))?;

        while let Some(chunk) = field.chunk().await.map_err(|e| {
            format!(
                "An error occurred while retrieving chunks from the field: {}",
                e
            )
        })? {
            file.write_all(&chunk)
                .await
                .map_err(|e| format!("An error occurred while writing to the file: {}", e))?;
            tx.send(chunk.len())
                .map_err(|e| format!("Could not send message to Progress API side: {}", e))?;
        }
        event!(Level::INFO, "Finished upload: {}", filename);
    }
    Ok(StatusCode::OK)
}
