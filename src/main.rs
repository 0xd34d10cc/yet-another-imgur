#[macro_use]
extern crate diesel;

use std::sync::Arc;

use actix_web::{App, HttpServer};

use crate::storage::PostgresStorage;
use crate::service::{upload, preview};

mod schema;
mod storage;
mod models;
mod service;


fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be defined");

    let storage = PostgresStorage::new(&url)
        .expect("Failed to initialize sqlite storage");
    let state = Arc::new(storage);

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(upload::bind::<PostgresStorage>("/images/upload"))
            .service(preview::bind::<PostgresStorage>("/images"))
        })
        .bind("127.0.0.1:8080")?
        .run()
}
