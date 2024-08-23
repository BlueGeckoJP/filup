use axum::response::Html;
use tera::Context;

use crate::TEMPLATES;

pub async fn root() -> Html<String> {
    let view = TEMPLATES
        .t
        .get()
        .unwrap()
        .lock()
        .await
        .render("root.html", &Context::new())
        .expect("Failed to load TEMPLATES (root.html)");
    Html(view)
}
