use super::DatasetRepository;
use crate::models::dataset::Dataset;
use anyhow::Result;
use sqlx::query_as;

impl DatasetRepository {
    pub async fn list_datasets(&self) -> Result<Vec<Dataset>> {
        Ok(query_as!(Dataset, "SELECT * FROM datasets")
            .fetch_all(&*self.pool)
            .await?)
    }
}
