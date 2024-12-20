use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Serializer};

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
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

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Datapoint {
    pub id: i64,
    pub dataset_id: String,
    pub data_type: DataPointType,
    pub name: Option<String>,
    #[serde(serialize_with = "serialize_bytes_to_string")]
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub indexed_at: Option<NaiveDateTime>,
    pub hash: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct DatapointWithMetadata {
    #[sqlx(flatten)]
    #[serde(flatten)]
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

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct DatapointChunk {
    pub id: i64,
    pub datapoint_id: i64,
    #[serde(serialize_with = "serialize_bytes_to_string")]
    // #[serde(deserialize_with = "deserialize_string_to_bytes")]
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
}

impl DatapointChunk {
    pub fn text(&self) -> Result<String> {
        String::from_utf8(self.data.clone()).map_err(|_| anyhow!("Invalid UTF-8 data"))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatapointChunkView {
    pub id: i64,
    pub datapoint_id: i64,
    #[serde(serialize_with = "serialize_bytes_to_string")]
    pub data: Vec<u8>,
    pub created_at: String,
    pub datapoint: Datapoint,
}

impl From<(DatapointChunk, Datapoint)> for DatapointChunkView {
    fn from((chunk, datapoint): (DatapointChunk, Datapoint)) -> Self {
        Self {
            id: chunk.id,
            datapoint_id: chunk.datapoint_id,
            data: chunk.data,
            created_at: chunk.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            datapoint,
        }
    }
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

#[derive(Serialize)]
pub struct DatapointView {
    pub id: i64,
    pub name: Option<String>,
    pub content_preview: String,
    pub created_at: String,
}

impl From<Datapoint> for DatapointView {
    fn from(datapoint: Datapoint) -> Self {
        let content_preview = String::from_utf8(datapoint.data.clone())
            .unwrap_or_else(|_| "Invalid UTF-8 data".to_string())
            .chars()
            .take(100)
            .collect::<String>();

        Self {
            id: datapoint.id,
            name: datapoint.name,
            content_preview,
            created_at: datapoint.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
