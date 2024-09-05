use std::{io::ErrorKind, sync::Arc, time::Duration};

use axum::{
    extract::{Query, State},
    response::{sse::Event, Sse},
};
use futures_util::stream::Stream;
use serde::Deserialize;
use tokio::sync::broadcast::{self};
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};
use tracing::{event, Level};

use crate::AppState;

#[derive(Deserialize)]
pub struct FilenameQuery {
    filename: String,
}

pub async fn progress(
    State(app_state): State<Arc<AppState>>,
    Query(filename): Query<FilenameQuery>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let (original_tx, original_rx) = broadcast::channel(2);
    let rx = original_tx.subscribe();
    {
        app_state
            .prog_channels
            .lock()
            .await
            .insert(filename.filename.clone(), (original_tx, original_rx));
    }
    event!(
        Level::INFO,
        "RX | Waiting for Upload API: {}",
        filename.filename
    );

    let stream = BroadcastStream::new(rx)
        .timeout(Duration::from_secs(10))
        .map(|msg| match msg {
            Ok(message) => Ok(Event::default().data(message.unwrap().to_string())),
            Err(e) => Err(axum::Error::new(std::io::Error::new(
                ErrorKind::Other,
                format!("Error receiving message: {}", e),
            ))),
        });

    Sse::new(stream)
}
