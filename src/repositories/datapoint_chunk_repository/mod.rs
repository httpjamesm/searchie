use sqlx::SqlitePool;
use std::sync::Arc;

pub mod crud;

pub struct DatapointChunkRepository {
    pool: Arc<SqlitePool>,
}

impl DatapointChunkRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
} 