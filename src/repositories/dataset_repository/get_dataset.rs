use super::DatasetRepository;
use crate::models::dataset::Dataset;
use anyhow::Result;

impl DatasetRepository {
    pub async fn get_dataset(&self, dataset_id: &str) -> Result<Dataset> {
        let dataset = sqlx::query_as!(Dataset, "SELECT * FROM datasets WHERE id = ?", dataset_id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(dataset)
    }
}
