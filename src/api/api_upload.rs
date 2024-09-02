use axum::{extract::Multipart, http::StatusCode};
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::{event, instrument, Level};

use crate::{PROGRESS_CONTAINER, SAVE_DIR};

#[instrument]
pub async fn upload(mut multipart: Multipart) -> Result<(), StatusCode> {
    let tx = { PROGRESS_CONTAINER.get().unwrap().tx.lock().await.clone() };

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filename = match field.file_name() {
            Some(name) => name.to_owned(),
            None => return Err(StatusCode::BAD_REQUEST),
        };
        event!(Level::INFO, "Start receive {}", filename);

        let mut file = File::create(format!("{}/{}", SAVE_DIR.clone(), filename))
            .await
            .unwrap();

        while let Some(chunk) = field.chunk().await.unwrap() {
            file.write_all(&chunk).await.unwrap();
            tx.send(chunk.len()).unwrap();
        }
    }
    Ok(())
}
