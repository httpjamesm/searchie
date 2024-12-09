use crate::{handlers::dataset_handler::DatasetHandler, TEMPLATES};
use poem::{
    handler,
    web::{Data, Html},
    Error,
};
use std::sync::Arc;
use tera::Context;

#[handler]
pub async fn dashboard_page(
    Data(dataset_handler): Data<&Arc<DatasetHandler>>,
) -> Result<Html<String>, Error> {
    let mut context = Context::new();

    let datasets = dataset_handler
        .dataset_controller
        .list_datasets()
        .await
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    context.insert("datasets", &datasets);

    TEMPLATES
        .render("dashboard/dashboard.html.tera", &context)
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(Html)
}
