use axum::response::Html;
use serde::Serialize;
use tera::Context;
use tokio::fs;

use crate::TEMPLATES;

#[derive(Debug, Serialize)]
struct FileItem {
    filename: String,
    path: String,
}

pub async fn root() -> Html<String> {
    let mut entries = fs::read_dir("./files").await.unwrap();
    let mut file_list_vec: Vec<FileItem> = vec![];
    while let Some(file) = entries.next_entry().await.unwrap() {
        let filename = file.file_name().to_string_lossy().to_string();
        let path = format!("/files/{}", filename);
        let item = FileItem { filename, path };
        file_list_vec.push(item);
    }

    let mut context = Context::new();
    context.insert("file_list", &file_list_vec);

    let view = TEMPLATES
        .t
        .get()
        .unwrap()
        .lock()
        .await
        .render("root.html", &context)
        .expect("Failed to load TEMPLATES (root.html)");
    Html(view)
}
