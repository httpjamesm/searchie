use anyhow::Result;
use poem::{listener::TcpListener, Route, Server};
use sqlx::sqlite::SqlitePoolOptions;

mod controllers;
mod handlers;
mod models;
mod repositories;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://./searchie.db")
        .await
        .expect("Failed to connect to database");

    let listen = "0.0.0.0:3030";
    println!("😼 Listening on {}", listen);
    let listener = TcpListener::bind(listen);

    let app = Route::new();
    Server::new(listener).run(app).await?;

    Ok(())
}
