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
        let created_at = Utc::now();
        let data_type = data_type.to_string();
        let id = query!(
            "INSERT INTO datapoints (dataset_id, data_type, name, data, created_at) VALUES (?, ?, ?, ?, ?)",
            dataset_id,
            data_type,
            name,
            data,
            created_at,
        )
        .execute(&*self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }
}
