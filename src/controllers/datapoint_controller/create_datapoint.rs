use std::collections::HashMap;

use crate::models::datapoint::DataPointType;
use crate::utils::chunking::chunk_text;
use anyhow::Result;
use small_world_rs::primitives::vector::Vector;

use super::DatapointController;

impl DatapointController {
    pub async fn create_datapoint(
        &self,
        dataset_id: &str,
        data_type: DataPointType,
        data: Vec<u8>,
        name: Option<String>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<i64> {
        let datapoint_id = self
            .datapoint_repository
            .create_datapoint(dataset_id, data_type, name.as_deref(), &data)
            .await?;

        if let Some(metadata) = metadata {
            for (key, value) in metadata {
                self.datapoint_metadata_repository
                    .create(datapoint_id, &key, &value)
                    .await?;
            }
        }

        self.indexing_queue_repository.enqueue(datapoint_id).await?;

        Ok(datapoint_id)
    }

    pub async fn index_datapoint(&self, datapoint_id: i64) -> Result<()> {
        let datapoint = self
            .datapoint_repository
            .get_datapoint(datapoint_id)
            .await?;

        let chunks = chunk_text(
            datapoint.name.as_deref(),
            &String::from_utf8(datapoint.data)?,
            256,
            128,
            &self.tokenizer,
        )?;

        let mut chunk_ids = Vec::new();
        let mut embeddings_batch = Vec::new();

        for chunk in chunks {
            let chunk_to_index = chunk.clone();
            let mut chunk_to_log = chunk.clone();
            if let Some(name) = datapoint.name.as_deref() {
                chunk_to_log = chunk_to_log.replacen(&format!("{} - ", name), "", 1);
            }
            println!("indexing chunk: {}", chunk_to_log);
            let chunk_id = self
                .datapoint_chunk_repository
                .create(datapoint_id, &chunk_to_log.as_bytes().to_vec())
                .await?;

            let embedding = self
                .embeddings_service
                .get_text_embedding(&chunk_to_index)
                .await?;
            embeddings_batch.push(embedding);
            chunk_ids.push(chunk_id);
        }

        for (i, chunk_id) in chunk_ids.iter().enumerate() {
            match self.worlds.lock().await.get_mut(&datapoint.dataset_id) {
                Some(world) => {
                    world.insert_vector(*chunk_id as u32, Vector::new_f16(&embeddings_batch[i]))?;
                }
                None => return Err(anyhow::anyhow!("World not found")),
            }
        }

        std::fs::write(
            &format!("indices/{}.smallworld", datapoint.dataset_id),
            self.worlds
                .lock()
                .await
                .get_mut(&datapoint.dataset_id)
                .unwrap()
                .dump()
                .unwrap(),
        )
        .unwrap();

        self.datapoint_repository.set_indexed(datapoint_id).await?;
        Ok(())
    }
}
