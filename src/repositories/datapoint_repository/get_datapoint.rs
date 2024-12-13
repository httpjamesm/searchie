use super::DatapointRepository;
use crate::models::datapoint::Datapoint;
use anyhow::Result;
use sqlx::{query, query_as, sqlite::SqliteRow, Row};

impl DatapointRepository {
    pub async fn get_datapoint(&self, id: i64) -> Result<Datapoint> {
        let datapoint = query_as!(Datapoint, "SELECT * FROM datapoints WHERE id = ?", id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(datapoint)
    }

    pub async fn get_datapoints(&self, dataset_id: &str) -> Result<Vec<Datapoint>> {
        let datapoints = query_as!(
            Datapoint,
            "SELECT * FROM datapoints WHERE dataset_id = ?",
            dataset_id
        )
        .fetch_all(&*self.pool)
        .await?;
        Ok(datapoints)
    }

    pub async fn get_datapoints_by_ids(&self, ids: Vec<i64>) -> Result<Vec<Datapoint>> {
        let placeholders = vec!["?"].repeat(ids.len()).join(",");
        let query = format!("SELECT * FROM datapoints WHERE id IN ({})", placeholders);
        let mut query = sqlx::query(&query);
        for id in ids {
            query = query.bind(id);
        }
        let rows = query.fetch_all(&*self.pool).await?;

        let mut datapoints = Vec::new();
        for row in rows {
            datapoints.push(Datapoint {
                id: row.get("id"),
                dataset_id: row.get("dataset_id"),
                data_type: row.get("data_type"),
                name: row.get("name"),
                data: row.get("data"),
                created_at: row.get("created_at"),
                indexed_at: row.get("indexed_at"),
                hash: row.get("hash"),
            });
        }
        Ok(datapoints)
    }
}
