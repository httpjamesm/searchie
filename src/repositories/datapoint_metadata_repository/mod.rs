use sqlx::SqlitePool;
use std::sync::Arc;

pub mod crud;

pub struct DatapointMetadataRepository {
    pool: Arc<SqlitePool>,
}

impl DatapointMetadataRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}
