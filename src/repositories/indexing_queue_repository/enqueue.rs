use anyhow::Result;

use super::IndexingQueueRepository;

impl IndexingQueueRepository {
    pub async fn enqueue(&self, datapoint_id: i64) -> Result<i64> {
        let result = sqlx::query!(
            "INSERT INTO indexing_queue (datapoint_id) VALUES (?) RETURNING id",
            datapoint_id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(result.id.expect("Failed to get inserted id"))
    }
}
