use super::DatasetRepository;
use anyhow::Result;
use chrono::Utc;
use sqlx::query;
use uuid::Uuid;

impl DatasetRepository {
    pub async fn create_dataset(&self, name: &str) -> Result<()> {
        let dataset_id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        query!(
            "INSERT INTO datasets (id, name, created_at) VALUES (?, ?, ?)",
            dataset_id,
            name,
            created_at,
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}
