use anyhow::Result;

use super::IndexingQueueRepository;

impl IndexingQueueRepository {
    pub async fn mark_failed(&self, id: i64, error: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE indexing_queue SET status = 'failed', error = ?, completed_at = CURRENT_TIMESTAMP WHERE id = ?",
            error,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
