use super::DatasetController;
use crate::models::datapoint::DatapointChunk;
use anyhow::Result;
use small_world_rs::primitives::vector::Vector;

impl DatasetController {
    pub async fn search_dataset(
        &self,
        dataset_id: &str,
        query: &str,
    ) -> Result<Vec<DatapointChunk>> {
        let dataset = self.dataset_repository.get_dataset(dataset_id).await?;

        // get world
        let worlds = self.worlds.lock().await;
        let world = worlds.get(&dataset.id).unwrap();

        // generate embedding of query
        let query_embedding = self.embeddings_service.get_text_embedding(query).await?;

        // search world
        let results = world.search(&Vector::new_f16(&query_embedding), 10, 20)?;

        println!("results: {:?}", results);

        // for every id, get the data point chunk from db
        let datapoint_chunks = self
            .datapoint_chunk_repository
            .get_by_ids(results.iter().map(|r| *r as i64).collect())
            .await?;

        Ok(datapoint_chunks)
    }
}
