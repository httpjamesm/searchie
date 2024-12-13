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
    match dataset_handler
        .dataset_controller
        .search_dataset(&dataset_id, &params.get("q").unwrap())
        .await
    {
        Ok(datapoints) => {
            let response = serde_json::json!({
                "datapoints": datapoints.0,
                "chunk_results": datapoints.1
            });
            Json(response).into_response()
        }
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string()),
    }
}
