use std::io::ErrorKind;

use axum::response::{sse::Event, Sse};
use futures_util::stream::Stream;
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};

use crate::PROGRESS_CONTAINER;

pub async fn progress() -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let rx = {
        PROGRESS_CONTAINER
            .get()
            .unwrap()
            .tx
            .lock()
            .await
            .subscribe()
    };

    let stream = BroadcastStream::new(rx).map(|msg| match msg {
        Ok(message) => Ok(Event::default().data(message.to_string())),
        Err(e) => Err(axum::Error::new(std::io::Error::new(
            ErrorKind::Other,
            format!("Error receiving message: {}", e),
        ))),
    });

    Sse::new(stream)
}
