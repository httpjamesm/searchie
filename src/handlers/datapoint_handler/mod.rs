use std::sync::Arc;

use crate::controllers::datapoint_controller::DatapointController;

pub mod create_datapoint;

pub struct DatapointHandler {
    datapoint_controller: Arc<DatapointController>,
}

impl DatapointHandler {
    pub fn new(datapoint_controller: Arc<DatapointController>) -> Self {
        Self {
            datapoint_controller,
        }
    }
}
