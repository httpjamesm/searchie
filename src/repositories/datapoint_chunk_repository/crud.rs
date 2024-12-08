use super::DatapointChunkRepository;
use crate::models::datapoint::DatapointChunk;
use anyhow::Result;
use sqlx::{query, query_as};

impl DatapointChunkRepository {
    pub async fn create(&self, datapoint_id: i64, data: &Vec<u8>) -> Result<i64> {
        let id = query!(
            "INSERT INTO datapoint_chunks (datapoint_id, data) VALUES (?, ?)",
            datapoint_id,
            data
        )
        .execute(&*self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn get_for_datapoint(&self, datapoint_id: i64) -> Result<Vec<DatapointChunk>> {
        let chunks = query_as!(
            DatapointChunk,
            "SELECT * FROM datapoint_chunks WHERE datapoint_id = ? ORDER BY created_at ASC",
            datapoint_id
        )
        .fetch_all(&*self.pool)
        .await?;
        Ok(chunks)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        query!("DELETE FROM datapoint_chunks WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_for_datapoint(&self, datapoint_id: i64) -> Result<()> {
        query!(
            "DELETE FROM datapoint_chunks WHERE datapoint_id = ?",
            datapoint_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_by_ids(&self, ids: Vec<i64>) -> Result<Vec<DatapointChunk>> {
        // Create placeholders for the IN clause (?,?,?)
        let placeholders = vec!["?"].repeat(ids.len()).join(",");
        let query = format!(
            "SELECT * FROM datapoint_chunks WHERE id IN ({})",
            placeholders
        );

        // Convert the query to a sqlx::Query
        let mut query = sqlx::query_as(&query);

        // Bind each ID as a separate parameter
        for id in ids {
            query = query.bind(id);
        }

        let chunks = query.fetch_all(&*self.pool).await?;

        Ok(chunks)
    }
}
