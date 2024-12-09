use crate::{
    handlers::dataset_handler::DatasetHandler, models::datapoint::DatapointView, TEMPLATES,
};
use poem::{
    handler,
    web::{Data, Html, Path},
    Error,
};
use std::sync::Arc;
use tera::Context;

#[handler]
pub async fn datapoints_page(
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

    let datapoints = dataset_handler
        .datapoint_controller
        .list_datapoints(&dataset_id)
        .await
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    let datapoint_views: Vec<DatapointView> =
        datapoints.into_iter().map(DatapointView::from).collect();

    context.insert("dataset", &dataset);
    context.insert("datapoints", &datapoint_views);

    TEMPLATES
        .render("dashboard/dataset/datapoints.html.tera", &context)
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(Html)
}
