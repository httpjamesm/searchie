use sqlx::{Pool, Sqlite};
use std::sync::Arc;

pub mod enqueue;
pub mod get_next_pending;
pub mod mark_completed;
pub mod mark_failed;
pub mod mark_processing;

pub struct IndexingQueueRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl IndexingQueueRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}
