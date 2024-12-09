use super::DatapointController;
use crate::models::datapoint::Datapoint;
use anyhow::Result;

impl DatapointController {
    pub async fn list_datapoints(&self, dataset_id: &str) -> Result<Vec<Datapoint>> {
        let datapoints = self.datapoint_repository.get_datapoints(dataset_id).await?;
        Ok(datapoints)
    }
}
