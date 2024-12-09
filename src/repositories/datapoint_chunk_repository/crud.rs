use super::DatapointChunkRepository;
use crate::models::datapoint::{
    Datapoint, DatapointChunk, DatapointChunkWithDatapoint, DatapointMetadata,
    DatapointWithMetadata,
};
use anyhow::Result;
use serde_json::{self, Value};
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

    pub async fn get_by_ids(&self, ids: Vec<i64>) -> Result<Vec<DatapointChunkWithDatapoint>> {
        let placeholders = vec!["?"].repeat(ids.len()).join(",");
        let query = format!(
            r#"
        SELECT 
            dc.id, dc.datapoint_id, dc.data as chunk_data, dc.created_at,
            d.id as d_id, d.dataset_id, d.data_type, d.name, d.data, d.created_at as d_created_at, d.indexed_at, d.hash,
            dm.id as dm_id, dm.datapoint_id as dm_datapoint_id, dm.key as dm_key, dm.value as dm_value, dm.created_at as dm_created_at
        FROM datapoint_chunks dc
        JOIN datapoints d ON dc.datapoint_id = d.id
        LEFT JOIN datapoint_metadata dm ON d.id = dm.datapoint_id
        WHERE dc.id IN ({})
        "#,
            placeholders
        );

        let mut query = sqlx::query(&query);
        for id in ids {
            query = query.bind(id);
        }

        let rows = query.fetch_all(&*self.pool).await?;

        let mut chunks_map: std::collections::HashMap<i64, DatapointChunkWithDatapoint> =
            std::collections::HashMap::new();

        for row in rows {
            let chunk_id: i64 = row.get("id");

            let chunk = chunks_map
                .entry(chunk_id)
                .or_insert_with(|| DatapointChunkWithDatapoint {
                    id: row.get("id"),
                    datapoint_id: row.get("datapoint_id"),
                    data: row.get("chunk_data"),
                    created_at: row.get("created_at"),
                    datapoint: DatapointWithMetadata {
                        datapoint: Datapoint {
                            id: row.get("d_id"),
                            dataset_id: row.get("dataset_id"),
                            data_type: row.get("data_type"),
                            name: row.get("name"),
                            data: row.get("data"),
                            created_at: row.get("d_created_at"),
                            indexed_at: row.get("indexed_at"),
                            hash: row.get("hash"),
                        },
                        metadata: Vec::new(),
                    },
                });

            if let (
                Ok(Some(metadata_id)),
                Ok(Some(datapoint_id)),
                Ok(Some(key)),
                Ok(Some(value)),
                Ok(Some(created_at)),
            ) = (
                row.try_get("dm_id"),
                row.try_get("dm_datapoint_id"),
                row.try_get("dm_key"),
                row.try_get("dm_value"),
                row.try_get("dm_created_at"),
            ) {
                chunk.datapoint.metadata.push(DatapointMetadata {
                    id: metadata_id,
                    datapoint_id,
                    key,
                    value,
                    created_at,
                });
            }
        }

        Ok(chunks_map.into_values().collect())
    }
}
