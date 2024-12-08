use super::DatasetController;
use anyhow::Result;
impl DatasetController {
    pub async fn create_dataset(&self, name: &str) -> Result<String> {
        // force name to be lowercase
        let dataset_id = self
            .dataset_repository
            .create_dataset(name.to_lowercase().as_str())
            .await?;
        Ok(dataset_id)
    }
}
