use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "indexing_status", rename_all = "lowercase")]
pub enum IndexingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl FromStr for IndexingStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(IndexingStatus::Pending),
            "processing" => Ok(IndexingStatus::Processing),
            "completed" => Ok(IndexingStatus::Completed),
            "failed" => Ok(IndexingStatus::Failed),
            _ => Err(anyhow::anyhow!("Invalid indexing status: {}", s)),
        }
    }
}

impl Display for IndexingStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingStatus::Pending => write!(f, "pending"),
            IndexingStatus::Processing => write!(f, "processing"),
            IndexingStatus::Completed => write!(f, "completed"),
            IndexingStatus::Failed => write!(f, "failed"),
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct IndexingQueueItem {
    pub id: i64,
    pub datapoint_id: i64,
    pub status: IndexingStatus,
    pub error: Option<String>,
    pub created_at: NaiveDateTime,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
}
