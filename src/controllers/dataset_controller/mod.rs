use std::sync::Arc;

use crate::repositories::dataset_repository::DatasetRepository;

pub mod create_dataset;
pub struct DatasetController {
    dataset_repository: Arc<DatasetRepository>,
}

impl DatasetController {
    pub fn new(dataset_repository: Arc<DatasetRepository>) -> Self {
        Self { dataset_repository }
    }
}
