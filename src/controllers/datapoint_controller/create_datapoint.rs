use super::DatapointController;
use crate::{models::datapoint::DataPointType, utils::chunking::chunk_text};
use anyhow::Result;
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

        for chunk in chunks {
            self.datapoint_chunk_repository
                .create(datapoint_id, &chunk.as_bytes().to_vec())
                .await?;
        }

        // update the datapoint to be indexed
        self.datapoint_repository.set_indexed(datapoint_id).await?;
        Ok(())
    }
}
