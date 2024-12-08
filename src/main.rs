use std::sync::Arc;

use anyhow::Result;
use controllers::{
    datapoint_controller::DatapointController, dataset_controller::DatasetController,
};
use handlers::{
    datapoint_handler::{create_datapoint::create_datapoint, DatapointHandler},
    dataset_handler::{create_dataset::create_dataset, DatasetHandler},
};
use poem::{listener::TcpListener, post, Route, Server};
use repositories::{
    datapoint_chunk_repository::DatapointChunkRepository,
    datapoint_metadata_repository::DatapointMetadataRepository,
    datapoint_repository::DatapointRepository, dataset_repository::DatasetRepository,
    services::embeddings::OllamaEmbeddingsService,
};
use sqlx::sqlite::SqlitePoolOptions;

mod controllers;
mod handlers;
mod models;
mod repositories;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = Arc::new(
        SqlitePoolOptions::new()
            .connect("sqlite://./searchie.db")
            .await
            .expect("Failed to connect to database"),
    );

    let listen = "0.0.0.0:3030";
    let listener = TcpListener::bind(listen);

    // services
    let tokenizer_path = "./tokenizer.json";
    let ollama_embeddings_service = Arc::new(OllamaEmbeddingsService::new());

    // repositories
    let datapoint_repository = Arc::new(DatapointRepository::new(pool.clone()));
    let dataset_repository = Arc::new(DatasetRepository::new(pool.clone()));
    let datapoint_metadata_repository = Arc::new(DatapointMetadataRepository::new(pool.clone()));
    let datapoint_chunk_repository = Arc::new(DatapointChunkRepository::new(pool.clone()));

    // controllers
    let datapoint_controller = Arc::new(DatapointController::new(
        datapoint_repository.clone(),
        datapoint_metadata_repository.clone(),
        datapoint_chunk_repository.clone(),
        ollama_embeddings_service.clone(),
        tokenizer_path,
    ));
    let dataset_controller = Arc::new(DatasetController::new(dataset_repository.clone()));

    // handlers
    let datapoint_handler = DatapointHandler::new(datapoint_controller.clone());
    let dataset_handler = DatasetHandler::new(dataset_controller.clone());

    let app = Route::new()
        .at("/datasets", post(create_dataset))
        .at("/datapoints", post(create_datapoint));

    println!("😼 Listening on {}", listen);

    Server::new(listener).run(app).await?;

    Ok(())
}
