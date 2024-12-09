use sqlx::SqlitePool;
use std::sync::Arc;

pub mod create_datapoint;
pub mod get_datapoint;
pub mod set_indexed;
pub mod count_datapoints;

pub struct DatapointRepository {
    pool: Arc<SqlitePool>,
}

impl DatapointRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}
