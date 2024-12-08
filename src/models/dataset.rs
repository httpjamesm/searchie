use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
