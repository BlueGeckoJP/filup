use axum::{extract::Multipart, http::StatusCode};
use base64::{engine::general_purpose, Engine};
use tokio::{fs::File, io::AsyncWriteExt, sync::broadcast::Sender};
use tracing::{event, Level};

use crate::{PROG_CH_LIST, SAVE_DIR};

pub async fn upload(mut multipart: Multipart) -> Result<(), StatusCode> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filename = match field.file_name() {
            Some(name) => name.to_owned(),
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let encoded_filename = general_purpose::URL_SAFE.encode(&filename).replace("=", "");
        let tx: Sender<usize>;
        {
            let hashmap = PROG_CH_LIST.get().unwrap().lock().await;
            let item = hashmap.get(&encoded_filename).unwrap();
            let original_tx = &item.0;
            tx = original_tx.clone();
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
