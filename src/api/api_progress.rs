use std::io::ErrorKind;

use axum::{
    extract::Query,
    response::{sse::Event, Sse},
};
use futures_util::stream::Stream;
use serde::Deserialize;
use tokio::sync::broadcast::Receiver;
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};

use crate::PROG_CH_LIST;

#[derive(Deserialize)]
pub struct FilenameQuery {
    filename: String,
}

pub async fn progress(
    Query(filename): Query<FilenameQuery>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let rx: Receiver<usize>;
    {
        let hashmap = PROG_CH_LIST.get().unwrap().lock().await;
        let item = hashmap.get(&filename.filename).unwrap();
        let tx = &item.0;
        rx = tx.subscribe();
    }

    let stream = BroadcastStream::new(rx).map(|msg| match msg {
        Ok(message) => Ok(Event::default().data(message.to_string())),
        Err(e) => Err(axum::Error::new(std::io::Error::new(
            ErrorKind::Other,
            format!("Error receiving message: {}", e),
        ))),
    });

    Sse::new(stream)
}
