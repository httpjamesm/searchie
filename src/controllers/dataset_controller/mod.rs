use crate::repositories::{
    datapoint_chunk_repository::DatapointChunkRepository, dataset_repository::DatasetRepository,
    services::embeddings::EmbeddingsService, services::reranking::RerankingService,
};
use small_world_rs::world::world::World;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub mod create_dataset;
pub mod get_dataset;
pub mod list_datasets;
pub mod search_dataset;

pub struct DatasetController {
    dataset_repository: Arc<DatasetRepository>,
    worlds: Arc<Mutex<HashMap<String, World>>>,
    embeddings_service: Arc<Box<dyn EmbeddingsService>>,
    datapoint_chunk_repository: Arc<DatapointChunkRepository>,
    reranking_service: Arc<Box<dyn RerankingService>>,
}

impl DatasetController {
    pub fn new(
        dataset_repository: Arc<DatasetRepository>,
        worlds: Arc<Mutex<HashMap<String, World>>>,
        embeddings_service: Arc<Box<dyn EmbeddingsService>>,
        datapoint_chunk_repository: Arc<DatapointChunkRepository>,
        reranking_service: Arc<Box<dyn RerankingService>>,
    ) -> Self {
        Self {
            dataset_repository,
            worlds,
            embeddings_service,
            datapoint_chunk_repository,
            reranking_service,
        }
    }
}
