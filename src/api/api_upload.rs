use axum::{extract::Multipart, http::StatusCode};
use base64::{engine::general_purpose, Engine};
use tokio::{fs::File, io::AsyncWriteExt, sync::broadcast};
use tracing::{event, Level};

use crate::{PROG_CH_LIST, SAVE_DIR};

pub async fn upload(mut multipart: Multipart) -> Result<(), StatusCode> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filename = match field.file_name() {
            Some(name) => name.to_owned(),
            None => return Err(StatusCode::BAD_REQUEST),
        };
        let base64_filename = general_purpose::STANDARD.encode(filename.clone());

        let (original_tx, original_rx) = broadcast::channel(2);
        let tx = original_tx.clone();
        {
            PROG_CH_LIST
                .get()
                .unwrap()
                .lock()
                .await
                .insert(base64_filename, (original_tx, original_rx));
        }

        event!(Level::INFO, "Starting upload: {}", filename);

        let mut file = File::create(format!("{}/{}", SAVE_DIR.clone(), filename))
            .await
            .unwrap();

        while let Some(chunk) = field.chunk().await.unwrap() {
            file.write_all(&chunk).await.unwrap();
            tx.send(chunk.len()).unwrap();
        }
        event!(Level::INFO, "Finished upload: {}", filename);
    }
    Ok(())
}
