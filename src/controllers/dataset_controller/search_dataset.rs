use super::DatasetController;
use crate::models::datapoint::{Datapoint, DatapointChunk};
use anyhow::Result;
use small_world_rs::primitives::vector::Vector;

impl DatasetController {
    pub async fn search_dataset(
        &self,
        dataset_id: &str,
        query: &str,
    ) -> Result<(Vec<DatapointChunk>, Vec<Datapoint>)> {
        let dataset = self.dataset_repository.get_dataset(dataset_id).await?;

        // get world
        let worlds = self.worlds.lock().await;
        let world = worlds.get(&dataset.id).unwrap();

        // generate embedding of query
        let query_embedding = self.embeddings_service.get_text_embedding(query).await?;

        // search world
        let results = world.search(&Vector::new_f16(&query_embedding), 10, 20)?;

        // for every id, get the data point chunk from db
        let datapoint_chunks = self
            .datapoint_chunk_repository
            .get_by_ids(results.iter().map(|r| *r as i64).collect())
            .await?;

        let datapoint_chunk_text_strings: Vec<String> = datapoint_chunks
            .iter()
            .map(|chunk| chunk.text())
            .collect::<Result<_>>()?;

        let reranked = self
            .reranking_service
            .rerank(query, datapoint_chunk_text_strings)
            .await?;

        let datapoint_ids: Vec<i64> = reranked.iter().map(|r| *r as i64).collect();
        // ensure they're unique
        let datapoint_ids: Vec<i64> = datapoint_ids
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let datapoints = self
            .datapoint_repository
            .get_datapoints_by_ids(datapoint_ids)
            .await?;

        Ok((
            reranked
                .iter()
                .map(|r| datapoint_chunks[*r].clone())
                .collect(),
            datapoints,
        ))
    }
}
