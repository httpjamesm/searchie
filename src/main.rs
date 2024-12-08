use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use controllers::{
    datapoint_controller::DatapointController, dataset_controller::DatasetController,
};
use handlers::{
    datapoint_handler::{create_datapoint::create_datapoint, DatapointHandler},
    dataset_handler::{create_dataset::create_dataset, DatasetHandler},
};
use poem::{listener::TcpListener, post, EndpointExt, Route, Server};
use repositories::{
    datapoint_chunk_repository::DatapointChunkRepository,
    datapoint_metadata_repository::DatapointMetadataRepository,
    datapoint_repository::DatapointRepository, dataset_repository::DatasetRepository,
    services::embeddings::OllamaEmbeddingsService,
};
use small_world_rs::{
    distance_metric::{CosineDistance, DistanceMetric},
    world::world::World,
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
    let ollama_embeddings_service = Box::new(OllamaEmbeddingsService::new());

    // repositories
    let datapoint_repository = Arc::new(DatapointRepository::new(pool.clone()));
    let dataset_repository = Arc::new(DatasetRepository::new(pool.clone()));
    let datapoint_metadata_repository = Arc::new(DatapointMetadataRepository::new(pool.clone()));
    let datapoint_chunk_repository = Arc::new(DatapointChunkRepository::new(pool.clone()));

    let worlds = Arc::new(Mutex::new(HashMap::new()));

    // list datasets
    let datasets = dataset_repository.list_datasets().await?;
    for dataset in datasets {
        // check `indices` folder if `{dataset.id}.smallworld` exists.
        if std::path::Path::new(&format!("indices/{}.smallworld", dataset.id)).exists() {
            // read from dump
            let dump_data = std::fs::read(&format!("indices/{}.smallworld", dataset.id)).unwrap();
            let world = World::new_from_dump(&dump_data).unwrap();
            worlds.lock().unwrap().insert(dataset.id, world);
        } else {
            let world = World::new(24, 50, 40, DistanceMetric::Cosine(CosineDistance)).unwrap();
            // dump to file
            std::fs::write(
                &format!("indices/{}.smallworld", dataset.id),
                world.dump().unwrap(),
            )
            .unwrap();
            worlds.lock().unwrap().insert(dataset.id, world);
        }
    }

    // controllers
    let datapoint_controller = Arc::new(DatapointController::new(
        datapoint_repository.clone(),
        datapoint_metadata_repository.clone(),
        datapoint_chunk_repository.clone(),
        Arc::new(ollama_embeddings_service),
        tokenizer_path,
        worlds.clone(),
    ));
    let dataset_controller = Arc::new(DatasetController::new(dataset_repository.clone()));

    // handlers
    let datapoint_handler = Arc::new(DatapointHandler::new(datapoint_controller.clone()));
    let dataset_handler = Arc::new(DatasetHandler::new(dataset_controller.clone()));

    let app = Route::new()
        .at("/datasets", post(create_dataset))
        .at("/datapoints", post(create_datapoint))
        .data(datapoint_handler)
        .data(dataset_handler);

    println!("😼 Listening on {}", listen);

    Server::new(listener).run(app).await?;

    Ok(())
}
