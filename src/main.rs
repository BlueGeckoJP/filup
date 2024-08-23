mod route;

use std::error::Error;

use axum::{routing::get, Router};
use once_cell::sync::OnceCell;
use route::r_root::root;
use tera::Tera;
use tokio::{net::TcpListener, sync::Mutex};

pub static TEMPLATES: Templates = Templates { t: OnceCell::new() };

pub struct Templates {
    t: OnceCell<Mutex<Tera>>,
}

impl Templates {
    async fn update(&self) -> Result<(), Box<dyn Error>> {
        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => return Err(Box::new(e)),
        };
        {
            if self.t.get().is_none() {
                self.t.set(Mutex::new(tera)).unwrap();
            } else {
                let mut t = self.t.get().unwrap().lock().await;
                *t = tera;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    TEMPLATES.update().await.unwrap();

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
