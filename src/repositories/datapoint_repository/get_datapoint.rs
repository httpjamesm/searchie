use super::DatapointRepository;
use crate::models::datapoint::Datapoint;
use anyhow::Result;
use sqlx::query_as;

impl DatapointRepository {
    pub async fn get_datapoint(&self, id: i64) -> Result<Datapoint> {
        let datapoint = query_as!(Datapoint, "SELECT * FROM datapoints WHERE id = ?", id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(datapoint)
    }
}
