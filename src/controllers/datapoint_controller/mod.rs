use std::sync::Arc;

use tokenizers::Tokenizer;

use crate::repositories::{
    datapoint_chunk_repository::DatapointChunkRepository,
    datapoint_metadata_repository::DatapointMetadataRepository,
    datapoint_repository::DatapointRepository, services::embeddings::EmbeddingsService,
};

pub mod create_datapoint;

pub struct DatapointController {
    datapoint_repository: Arc<DatapointRepository>,
    datapoint_metadata_repository: Arc<DatapointMetadataRepository>,
    datapoint_chunk_repository: Arc<DatapointChunkRepository>,
    embeddings_service: Arc<dyn EmbeddingsService>,
    tokenizer: Arc<Tokenizer>,
}

impl DatapointController {
    pub fn new(
        datapoint_repository: Arc<DatapointRepository>,
        datapoint_metadata_repository: Arc<DatapointMetadataRepository>,
        datapoint_chunk_repository: Arc<DatapointChunkRepository>,
        embeddings_service: Arc<dyn EmbeddingsService>,
        tokenizer_path: &str,
    ) -> Self {
        let tokenizer = Arc::new(Tokenizer::from_file(tokenizer_path).unwrap());
        Self {
            datapoint_repository,
            datapoint_metadata_repository,
            datapoint_chunk_repository,
            embeddings_service,
            tokenizer,
        }
    }
}
