use sqlx::SqlitePool;
use std::sync::Arc;

pub mod create_dataset;

pub struct DatasetRepository {
    pool: Arc<SqlitePool>,
}

impl DatasetRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}
