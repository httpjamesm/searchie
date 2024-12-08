use super::DatapointMetadataRepository;
use crate::models::datapoint::DatapointMetadata;
use anyhow::Result;
use sqlx::{query, query_as};

impl DatapointMetadataRepository {
    pub async fn create(&self, datapoint_id: i64, key: &str, value: &str) -> Result<i64> {
        let id = query!(
            "INSERT INTO datapoint_metadata (datapoint_id, key, value) VALUES (?, ?, ?)",
            datapoint_id,
            key,
            value
        )
        .execute(&*self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_for_datapoint(&self, datapoint_id: i64) -> Result<Vec<DatapointMetadata>> {
        let metadata = query_as!(
            DatapointMetadata,
            "SELECT * FROM datapoint_metadata WHERE datapoint_id = ?",
            datapoint_id
        )
        .fetch_all(&*self.pool)
        .await?;
        Ok(metadata)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        query!("DELETE FROM datapoint_metadata WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}
