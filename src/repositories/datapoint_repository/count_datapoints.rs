use super::DatapointRepository;
use anyhow::Result;

impl DatapointRepository {
    pub async fn count_datapoints(&self, dataset_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count
            FROM datapoints
            WHERE dataset_id = $1
            "#,
            dataset_id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(count)
    }
}
