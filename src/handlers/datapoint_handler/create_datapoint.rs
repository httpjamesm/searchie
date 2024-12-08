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
    datapoints: Vec<CreateDatapointPayload>,
}

#[derive(Deserialize)]
pub struct CreateDatapointPayload {
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
    for datapoint in payload.datapoints {
        match datapoint_handler
            .datapoint_controller
            .create_datapoint(
                &payload.dataset_id,
                &datapoint.data_type,
                &datapoint.name,
                &datapoint.data.as_bytes().to_vec(),
                datapoint.metadata,
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(e.to_string())
            }
        }
    }
    Response::builder().status(StatusCode::CREATED).body(())
}
