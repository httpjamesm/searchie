use super::DatapointRepository;
use anyhow::Result;
use sqlx::query;

impl DatapointRepository {
    pub async fn set_indexed(&self, id: i64) -> Result<()> {
        query!(
            "UPDATE datapoints SET indexed_at = CURRENT_TIMESTAMP WHERE id = ?",
            id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}
