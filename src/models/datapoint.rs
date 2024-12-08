use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};

pub enum DataPointType {
    Text,
    // Text for now, but maybe in the future we'll have other types
}

impl DataPointType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(DataPointType::Text),
            _ => Err(anyhow!("Invalid data point type")),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            DataPointType::Text => "text",
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct Datapoint {
    pub id: u32,
    pub dataset_id: String,
    pub data_type: DataPointType,
    // we're storing the data as a blob to allow for future flexibility
    pub data: Vec<u8>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct DatapointMetadata {
    pub id: u32,
    pub datapoint_id: u32,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
}
