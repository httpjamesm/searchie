use super::DatasetHandler;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    IntoResponse, Response,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct CreateDatasetRequest {
    name: String,
}

#[handler]
pub async fn create_dataset(
    dataset_handler: Data<&Arc<DatasetHandler>>,
    Json(payload): Json<CreateDatasetRequest>,
) -> impl IntoResponse {
    match dataset_handler
        .dataset_controller
        .create_dataset(&payload.name)
        .await
    {
        Ok(_) => Response::builder().status(StatusCode::CREATED).body(()),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string()),
    }
}
