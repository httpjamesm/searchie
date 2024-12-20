use crate::controllers::{
    datapoint_controller::DatapointController, dataset_controller::DatasetController,
};
use std::sync::Arc;

pub mod create_dataset;
pub mod dashboard_page;
pub mod datapoints_page;
pub mod dataset_page;
pub mod home_page;
pub mod list_datasets;
pub mod search_dataset;
pub mod search_page;
pub mod upload_page;

#[derive(Clone)]
pub struct DatasetHandler {
    dataset_controller: Arc<DatasetController>,
    datapoint_controller: Arc<DatapointController>,
}

impl DatasetHandler {
    pub fn new(
        dataset_controller: Arc<DatasetController>,
        datapoint_controller: Arc<DatapointController>,
    ) -> Self {
        Self {
            dataset_controller,
            datapoint_controller,
        }
    }
}
