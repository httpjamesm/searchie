use crate::controllers::dataset_controller::DatasetController;
use crate::models::dataset::Dataset;
use anyhow::Result;

impl DatasetController {
    pub async fn get_dataset(&self, dataset_id: &str) -> Result<Dataset> {
        self.dataset_repository.get_dataset(dataset_id).await
    }
}
