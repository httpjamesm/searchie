use super::DatasetController;
use anyhow::Result;
impl DatasetController {
    pub async fn create_dataset(&self, name: &str) -> Result<String> {
        let dataset_id = self.dataset_repository.create_dataset(name).await?;
        Ok(dataset_id)
    }
}
