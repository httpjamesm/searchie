use crate::{handlers::dataset_handler::DatasetHandler, TEMPLATES};
use poem::{
    handler,
    web::{Data, Html, Path},
    Error,
};
use std::sync::Arc;
use tera::Context;

#[handler]
pub async fn upload_page(
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

    context.insert("dataset", &dataset);

    TEMPLATES
        .render("dashboard/dataset/upload.html.tera", &context)
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(Html)
}
