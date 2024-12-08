use sqlx::SqlitePool;
use std::sync::Arc;

pub mod create_dataset;
pub mod get_dataset;
pub mod list_datasets;

pub struct DatasetRepository {
    pool: Arc<SqlitePool>,
}

impl DatasetRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}
