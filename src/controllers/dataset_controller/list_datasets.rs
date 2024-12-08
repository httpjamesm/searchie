use super::DatasetController;
use crate::models::dataset::Dataset;
use anyhow::Result;

impl DatasetController {
    pub async fn list_datasets(&self) -> Result<Vec<Dataset>> {
        Ok(self.dataset_repository.list_datasets().await?)
    }
}
