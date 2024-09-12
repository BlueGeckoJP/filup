use std::sync::Arc;

use axum::{extract::State, Json};
use ripemd::Ripemd160;
use serde::Serialize;
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use tracing::{event, Level};

use crate::{path_check, AppState};

#[derive(Serialize)]
pub struct Hashes {
    sha256: String,
    ripemd160: String,
}

pub async fn hash(
    State(app_state): State<Arc<AppState>>,
    filename: String,
) -> Result<Json<Hashes>, String> {
    event!(Level::INFO, "Hash API: Opening file: {}", &filename);
    let mut file = BufReader::new(
        File::open(
            path_check(&app_state.save_dir, &filename)
                .await
                .map_err(|e| e.to_string())?,
        )
        .await
        .map_err(|e| format!("An error occurred while opening file: {}", e))?,
    );
    let mut file_buf = vec![];
    file.read_to_end(&mut file_buf)
        .await
        .map_err(|e| format!("An error occurred while reading file: {}", e))?;

    event!(Level::INFO, "Hash API: Generating SHA256: {}", &filename);
    let sha256_hash = Sha256::digest(&file_buf);
    let sha256_str = format!("{:x}", sha256_hash);

    event!(
        Level::INFO,
        "Hash API: Generating RIPEMD-160: {}",
        &filename
    );
    let mut ripemd160_hasher = Ripemd160::new();
    ripemd160_hasher.update(&file_buf);
    let ripemd160_str = format!("{:x}", ripemd160_hasher.finalize());

    Ok(Json(Hashes {
        sha256: sha256_str,
        ripemd160: ripemd160_str,
    }))
}
