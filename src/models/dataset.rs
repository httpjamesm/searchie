use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}
