use anyhow::Result;

use super::IndexingQueueRepository;
use crate::models::indexing_queue::{IndexingQueueItem, IndexingStatus};

impl IndexingQueueRepository {
    pub async fn get_next_pending(&self) -> Result<Option<IndexingQueueItem>> {
        let item = sqlx::query_as!(
            IndexingQueueItem,
            r#"
            SELECT 
                id,
                datapoint_id,
                status as "status: IndexingStatus",
                error,
                created_at,
                started_at,
                completed_at
            FROM indexing_queue 
            WHERE status = 'pending'
            ORDER BY created_at ASC 
            LIMIT 1
            "#
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(item)
    }
}
