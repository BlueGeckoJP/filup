use axum::{extract::Multipart, http::StatusCode};
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::{event, instrument, Level};

use crate::SAVE_DIR;

#[instrument]
pub async fn upload(mut multipart: Multipart) -> Result<(), StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = match field.file_name() {
            Some(name) => name.to_owned(),
            None => return Err(StatusCode::BAD_REQUEST),
        };
        match field.bytes().await {
            Ok(data) => {
                event!(Level::INFO, "Uploading {}, {}bytes", filename, data.len());
                let mut file = File::create(format!("{}/{}", SAVE_DIR.clone(), filename))
                    .await
                    .unwrap();
                file.write_all(&data).await.unwrap();
            }
            Err(e) => {
                event!(
                    Level::ERROR,
                    "An error occurred while receiving the uploaded file: Filename: {}, {}",
                    filename,
                    e
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
    Ok(())
}
