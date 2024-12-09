use anyhow::Result;

use super::IndexingQueueRepository;

impl IndexingQueueRepository {
    pub async fn mark_completed(&self, id: i64) -> Result<()> {
        sqlx::query!(
            "UPDATE indexing_queue SET status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = ?",
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
