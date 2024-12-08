use super::DatapointController;
use crate::{models::datapoint::DataPointType, utils::chunking::chunk_text};
use anyhow::Result;
use small_world_rs::primitives::vector::Vector;
use std::collections::HashMap;

impl DatapointController {
    pub async fn create_datapoint(
        &self,
        dataset_id: &str,
        data_type: &str,
        name: &str,
        data: &Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<()> {
        let datapoint_id = self
            .datapoint_repository
            .create_datapoint(
                dataset_id,
                DataPointType::from_str(data_type).unwrap(),
                name,
                data,
            )
            .await?;

        if let Some(metadata) = metadata {
            for (key, value) in metadata {
                self.datapoint_metadata_repository
                    .create(datapoint_id, &key, &value)
                    .await?;
            }
        }

        self.index_datapoint(datapoint_id).await?;
        Ok(())
    }

    pub async fn index_datapoint(&self, datapoint_id: i64) -> Result<()> {
        let datapoint = self
            .datapoint_repository
            .get_datapoint(datapoint_id)
            .await?;

        let chunks = chunk_text(
            &datapoint.name,
            &String::from_utf8(datapoint.data)?,
            1000,
            100,
            &self.tokenizer,
        )?;

        let mut chunk_ids = Vec::new();
        let mut embeddings_batch = Vec::new();

        for chunk in chunks {
            let chunk_id = self
                .datapoint_chunk_repository
                .create(datapoint_id, &chunk.as_bytes().to_vec())
                .await?;

            // create embedding
            let embedding = self.embeddings_service.get_text_embedding(&chunk).await?;
            embeddings_batch.push(embedding);
            chunk_ids.push(chunk_id);
        }

        for (i, chunk_id) in chunk_ids.iter().enumerate() {
            // insert into world
            match self.worlds.lock().unwrap().get_mut(&datapoint.dataset_id) {
                Some(world) => {
                    world.insert_vector(*chunk_id as u32, Vector::new_f16(&embeddings_batch[i]))?;
                }
                None => return Err(anyhow::anyhow!("World not found")),
            }
        }

        // dump the world to disk
        std::fs::write(
            &format!("indices/{}.smallworld", datapoint.dataset_id),
            self.worlds
                .lock()
                .unwrap()
                .get_mut(&datapoint.dataset_id)
                .unwrap()
                .dump()
                .unwrap(),
        )
        .unwrap();

        // update the datapoint to be indexed
        self.datapoint_repository.set_indexed(datapoint_id).await?;
        Ok(())
    }
}
