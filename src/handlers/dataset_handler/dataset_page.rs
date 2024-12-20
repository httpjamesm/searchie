use crate::{handlers::dataset_handler::DatasetHandler, TEMPLATES};
use poem::{
    handler,
    web::{Data, Html, Path},
    Error,
};
use std::sync::Arc;
use tera::Context;

#[handler]
pub async fn dataset_page(
    Data(dataset_handler): Data<&Arc<DatasetHandler>>,
    Path(dataset_id): Path<String>,
) -> Result<Html<String>, Error> {
    let mut context = Context::new();

    let dataset = dataset_handler
        .dataset_controller
        .get_dataset(&dataset_id)
        .await
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    let datapoint_count = dataset_handler
        .datapoint_controller
        .count_datapoints(&dataset_id)
        .await
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    context.insert("dataset", &dataset);
    context.insert("datapoint_count", &datapoint_count);

    TEMPLATES
        .render("dashboard/dataset/dataset.html.tera", &context)
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(Html)
}
