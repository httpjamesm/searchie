use crate::handlers::dataset_handler::DatasetHandler;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    IntoResponse, Response,
};
use std::sync::Arc;

#[handler]
pub async fn list_datasets(dataset_handler: Data<&Arc<DatasetHandler>>) -> Response {
    match dataset_handler.dataset_controller.list_datasets().await {
        Ok(datasets) => Json(datasets).into_response(),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string()),
    }
}
