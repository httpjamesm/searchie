use small_world_rs::world::world::World;
use std::{collections::HashMap, sync::Arc};
use tokenizers::Tokenizer;
use tokio::sync::Mutex;

use crate::{
    repositories::{
        datapoint_chunk_repository::DatapointChunkRepository,
        datapoint_metadata_repository::DatapointMetadataRepository,
        datapoint_repository::DatapointRepository,
        indexing_queue_repository::IndexingQueueRepository,
    },
    services::embeddings::EmbeddingsService,
};

pub mod create_datapoint;
pub mod list_datapoints;

pub struct DatapointController {
    datapoint_repository: Arc<DatapointRepository>,
    datapoint_metadata_repository: Arc<DatapointMetadataRepository>,
    datapoint_chunk_repository: Arc<DatapointChunkRepository>,
    embeddings_service: Arc<Box<dyn EmbeddingsService>>,
    tokenizer: Arc<Tokenizer>,
    worlds: Arc<Mutex<HashMap<String, World>>>,
    indexing_queue_repository: Arc<IndexingQueueRepository>,
}

impl DatapointController {
    pub fn new(
        datapoint_repository: Arc<DatapointRepository>,
        datapoint_metadata_repository: Arc<DatapointMetadataRepository>,
        datapoint_chunk_repository: Arc<DatapointChunkRepository>,
        embeddings_service: Arc<Box<dyn EmbeddingsService>>,
        tokenizer_path: &str,
        worlds: Arc<Mutex<HashMap<String, World>>>,
        indexing_queue_repository: Arc<IndexingQueueRepository>,
    ) -> Self {
        let tokenizer = Arc::new(Tokenizer::from_file(tokenizer_path).unwrap());
        Self {
            datapoint_repository,
            datapoint_metadata_repository,
            datapoint_chunk_repository,
            embeddings_service,
            tokenizer,
            worlds,
            indexing_queue_repository,
        }
    }
}
