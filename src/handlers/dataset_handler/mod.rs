use crate::controllers::dataset_controller::DatasetController;
use std::sync::Arc;

pub mod create_dataset;
pub mod list_datasets;
#[derive(Clone)]
pub struct DatasetHandler {
    dataset_controller: Arc<DatasetController>,
}

impl DatasetHandler {
    pub fn new(dataset_controller: Arc<DatasetController>) -> Self {
        Self { dataset_controller }
    }
}
