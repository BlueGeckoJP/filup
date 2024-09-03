use std::{io::ErrorKind, time::Duration};

use axum::{
    extract::Query,
    response::{sse::Event, Sse},
};
use futures_util::stream::Stream;
use serde::Deserialize;
use tokio::sync::broadcast::{self};
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};
use tracing::{event, Level};

use crate::PROG_CH_LIST;

#[derive(Deserialize)]
pub struct FilenameQuery {
    filename: String,
}

pub async fn progress(
    Query(filename): Query<FilenameQuery>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let (original_tx, original_rx) = broadcast::channel(2);
    let rx = original_tx.subscribe();
    {
        PROG_CH_LIST
            .get()
            .unwrap()
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
