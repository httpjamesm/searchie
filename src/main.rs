use anyhow::Result;
use controllers::{
    datapoint_controller::DatapointController, dataset_controller::DatasetController,
};
use handlers::{
    datapoint_handler::{create_datapoint::create_datapoint, DatapointHandler},
    dataset_handler::{
        create_dataset::create_dataset, dashboard_page::dashboard_page,
        datapoints_page::datapoints_page, dataset_page::dataset_page, home_page::home_page,
        search_dataset::search_dataset, search_page::search_page, upload_page::upload_page,
        DatasetHandler,
    },
};
use once_cell::sync::Lazy;
use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};
use repositories::{
    datapoint_chunk_repository::DatapointChunkRepository,
    datapoint_metadata_repository::DatapointMetadataRepository,
    datapoint_repository::DatapointRepository, dataset_repository::DatasetRepository,
    indexing_queue_repository::IndexingQueueRepository,
};
use services::{
    embeddings::{EmbeddingsService, OllamaEmbeddingsService},
    indexing_worker::IndexingWorker,
    reranking::{FastEmbedRerankingService, RerankingService},
};
use small_world_rs::{
    distance_metric::{CosineDistance, DistanceMetric},
    world::world::World,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::{collections::HashMap, sync::Arc};
use tera::Tera;
use tokio::sync::Mutex;

mod controllers;
mod handlers;
mod models;
mod repositories;
mod services;
mod utils;

static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {e}");
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html.tera"]);
    tera
});

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
    let ollama_embeddings_service =
        Arc::new(Box::new(OllamaEmbeddingsService::new()) as Box<dyn EmbeddingsService>);
    let fastembed_reranking_service = Arc::new(Box::new(FastEmbedRerankingService::new(
        "./.fastembed",
    )) as Box<dyn RerankingService>);

    // repositories
    let datapoint_repository = Arc::new(DatapointRepository::new(pool.clone()));
    let dataset_repository = Arc::new(DatasetRepository::new(pool.clone()));
    let datapoint_metadata_repository = Arc::new(DatapointMetadataRepository::new(pool.clone()));
    let datapoint_chunk_repository = Arc::new(DatapointChunkRepository::new(pool.clone()));
    let indexing_queue_repository = Arc::new(IndexingQueueRepository::new(pool.clone()));

    let worlds = Arc::new(Mutex::new(HashMap::new()));

    // list datasets
    let datasets = dataset_repository.list_datasets().await?;
    for dataset in datasets {
        // check `indices` folder if `{dataset.id}.smallworld` exists.
        if std::path::Path::new(&format!("indices/{}.smallworld", dataset.id)).exists() {
            // read from dump
            let dump_data = std::fs::read(&format!("indices/{}.smallworld", dataset.id)).unwrap();
            let world = World::new_from_dump(&dump_data).unwrap();
            worlds.lock().await.insert(dataset.id, world);
        } else {
            let world = World::new(24, 50, 40, DistanceMetric::Cosine(CosineDistance)).unwrap();
            // dump to file
            std::fs::write(
                &format!("indices/{}.smallworld", dataset.id),
                world.dump().unwrap(),
            )
            .unwrap();
            worlds.lock().await.insert(dataset.id, world);
        }
    }

    // controllers
    let datapoint_controller = Arc::new(DatapointController::new(
        datapoint_repository.clone(),
        datapoint_metadata_repository.clone(),
        datapoint_chunk_repository.clone(),
        ollama_embeddings_service.clone(),
        tokenizer_path,
        worlds.clone(),
        indexing_queue_repository.clone(),
    ));
    let dataset_controller = Arc::new(DatasetController::new(
        dataset_repository.clone(),
        worlds.clone(),
        ollama_embeddings_service.clone(),
        datapoint_chunk_repository.clone(),
        fastembed_reranking_service.clone(),
        datapoint_repository.clone(),
    ));

    // handlers
    let datapoint_handler = Arc::new(DatapointHandler::new(datapoint_controller.clone()));
    let dataset_handler = Arc::new(DatasetHandler::new(
        dataset_controller.clone(),
        datapoint_controller.clone(),
    ));

    let indexing_worker = IndexingWorker::new(
        indexing_queue_repository.clone(),
        datapoint_controller.clone(),
    );

    tokio::spawn(async move {
        indexing_worker.start().await;
    });

    let app = Route::new()
        .nest(
            "/api",
            Route::new()
                .at("/datasets", post(create_dataset))
                .at("/datasets/:id/search", get(search_dataset))
                .at("/datapoints", post(create_datapoint)),
        )
        .nest(
            "/dashboard",
            Route::new()
                .at("/", get(dashboard_page))
                .at("/datasets/:id", get(dataset_page))
                .at("/datasets/:id/datapoints", get(datapoints_page))
                .at("/datasets/:id/upload", get(upload_page)),
        )
        .at("/", get(home_page))
        .at("/:id/search", get(search_page))
        .data(datapoint_handler)
        .data(dataset_handler);

    println!("😼 Listening on {}", listen);

    Server::new(listener).run(app).await?;

    Ok(())
}
