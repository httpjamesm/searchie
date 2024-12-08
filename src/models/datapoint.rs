use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "data_point_type", rename_all = "lowercase")]
pub enum DataPointType {
    Text,
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

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Datapoint {
    pub id: i64,
    pub dataset_id: String,
    pub data_type: DataPointType,
    pub name: Option<String>,
    #[serde(serialize_with = "serialize_bytes_to_string")]
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub indexed_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct DatapointWithMetadata {
    #[sqlx(flatten)]
    pub datapoint: Datapoint,
    pub metadata: Vec<DatapointMetadata>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct DatapointMetadata {
    pub id: i64,
    pub datapoint_id: i64,
    pub key: String,
    pub value: String,
    pub created_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct DatapointChunk {
    pub id: i64,
    pub datapoint_id: i64,
    #[serde(serialize_with = "serialize_bytes_to_string")]
    // #[serde(deserialize_with = "deserialize_string_to_bytes")]
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct DatapointChunkWithDatapoint {
    pub id: i64,
    pub datapoint_id: i64,
    #[serde(serialize_with = "serialize_bytes_to_string")]
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub datapoint: DatapointWithMetadata,
}

fn serialize_bytes_to_string<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match String::from_utf8(bytes.clone()) {
        Ok(s) => serializer.serialize_str(&s),
        Err(_) => serializer.serialize_str("Invalid UTF-8 data"),
    }
}

// fn deserialize_string_to_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     Ok(s.into_bytes())
// }
