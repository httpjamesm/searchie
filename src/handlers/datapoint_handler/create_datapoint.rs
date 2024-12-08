use super::DatapointHandler;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    IntoResponse, Response,
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

#[derive(Deserialize)]
pub struct CreateDatapointRequest {
    dataset_id: String,
    data_type: String,
    name: String,
    data: String,
    metadata: Option<HashMap<String, String>>,
}

#[handler]
pub async fn create_datapoint(
    datapoint_handler: Data<&Arc<DatapointHandler>>,
    Json(payload): Json<CreateDatapointRequest>,
) -> impl IntoResponse {
    match datapoint_handler
        .datapoint_controller
        .create_datapoint(
            &payload.dataset_id,
            &payload.data_type,
            &payload.name,
            &payload.data.as_bytes().to_vec(),
            payload.metadata,
        )
        .await
    {
        Ok(_) => Response::builder().status(StatusCode::CREATED).body(()),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string()),
    }
}
