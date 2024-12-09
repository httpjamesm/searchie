use super::DatapointRepository;
use crate::models::datapoint::DataPointType;
use anyhow::Result;
use chrono::Utc;
use sqlx::query;

impl DatapointRepository {
    pub async fn create_datapoint(
        &self,
        dataset_id: &str,
        data_type: DataPointType,
        name: Option<&str>,
        data: &Vec<u8>,
    ) -> Result<i64> {
        // hash the data with blake3
        let hash = blake3::hash(data);
        let hash_hex = hash.to_hex();
        let hash_hex_string = hash_hex.to_string();

        let created_at = Utc::now();
        let data_type = data_type.to_string();
        let id = query!(
            "INSERT INTO datapoints (dataset_id, data_type, name, data, created_at, hash) VALUES (?, ?, ?, ?, ?, ?)",
            dataset_id,
            data_type,
            name,
            data,
            created_at,
            hash_hex_string
        )
        .execute(&*self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }
}
