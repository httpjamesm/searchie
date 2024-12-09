use crate::controllers::dataset_controller::DatasetController;
use std::sync::Arc;

pub mod create_dataset;
pub mod list_datasets;
pub mod search_dataset;
pub mod search_page;
pub mod dashboard_page;

#[derive(Clone)]
pub struct DatasetHandler {
    dataset_controller: Arc<DatasetController>,
}

impl DatasetHandler {
    pub fn new(dataset_controller: Arc<DatasetController>) -> Self {
        Self { dataset_controller }
    }
}
