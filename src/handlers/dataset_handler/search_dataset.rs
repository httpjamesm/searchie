use crate::handlers::dataset_handler::DatasetHandler;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path, Query},
    IntoResponse, Response,
};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

#[handler]
pub async fn search_dataset(
    dataset_handler: Data<&Arc<DatasetHandler>>,
    Path(dataset_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let limit = params
        .get("limit")
        .map(|l| l.parse::<usize>().unwrap_or(10))
        .unwrap_or(10);
    let query = match params.get("q") {
        Some(q) => q,
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Query is required")
                .into_response()
        }
    };
    match dataset_handler
        .dataset_controller
        .search_dataset(&dataset_id, query, limit)
        .await
    {
        Ok(datapoints) => {
            let response = serde_json::json!({
                "chunk_results": datapoints.0,
                "datapoints": datapoints.1
            });
            Json(response).into_response()
        }
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string()),
    }
}
