use super::DatapointChunkRepository;
use crate::models::datapoint::{
    Datapoint, DatapointChunk, DatapointMetadata, DatapointWithMetadata,
};
use anyhow::Result;
use sqlx::{query, query_as, Row};

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
        let placeholders = vec!["?"].repeat(ids.len()).join(",");
        let query = format!(
            r#"
        SELECT * FROM datapoint_chunks WHERE id IN ({})
        "#,
            placeholders
        );

        let mut query = sqlx::query(&query);
        for id in ids {
            query = query.bind(id);
        }

        let rows = query.fetch_all(&*self.pool).await?;

        let mut chunks_map: std::collections::HashMap<i64, DatapointChunk> =
            std::collections::HashMap::new();

        for row in rows {
            let chunk_id: i64 = row.get("id");

            chunks_map.entry(chunk_id).or_insert(DatapointChunk {
                id: row.get("id"),
                datapoint_id: row.get("datapoint_id"),
                data: row.get("data"),
                created_at: row.get("created_at"),
            });
        }

        Ok(chunks_map.into_values().collect())
    }
}
