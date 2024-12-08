use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;

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

impl From<String> for DataPointType {
    fn from(value: String) -> Self {
        DataPointType::from_str(&value).unwrap()
    }
}

#[derive(sqlx::FromRow)]
pub struct Datapoint {
    pub id: i64,
    pub dataset_id: String,
    pub data_type: DataPointType,
    pub name: String,
    // we're storing the data as a blob to allow for future flexibility
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub indexed_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow)]
pub struct DatapointMetadata {
    pub id: i64,
    pub datapoint_id: i64,
    pub key: String,
    pub value: String,
    pub created_at: NaiveDateTime,
}
#[derive(sqlx::FromRow)]
pub struct DatapointChunk {
    pub id: i64,
    pub datapoint_id: i64,
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
}
