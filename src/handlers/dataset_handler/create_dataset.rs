use super::DatasetHandler;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    IntoResponse, Response,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
struct CreateDatasetRequest {
    name: String,
}

#[handler]
pub async fn create_dataset(
    dataset_handler: Data<&Arc<DatasetHandler>>,
    Json(payload): Json<CreateDatasetRequest>,
) -> Response {
    match dataset_handler
        .dataset_controller
        .create_dataset(&payload.name)
        .await
    {
        Ok(id) => Json(json!({
            "message": "Dataset created successfully",
            "data": { "id": id }
        }))
        .into_response(),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string()),
    }
}
