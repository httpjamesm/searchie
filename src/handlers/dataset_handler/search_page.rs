use crate::{handlers::dataset_handler::DatasetHandler, TEMPLATES};
use poem::{
    error::InternalServerError,
    handler,
    web::{Data, Html, Path, Query},
    Error, IntoResponse, Response,
};
use std::{collections::HashMap, sync::Arc};
use tera::Context;

#[handler]
pub async fn search_page(
    Data(dataset_handler): Data<&Arc<DatasetHandler>>,
    Path(dataset_id): Path<String>,
    Query(query_params): Query<HashMap<String, String>>,
) -> Result<Html<String>, Error> {
    let mut context = Context::new();
    context.insert("dataset_id", &dataset_id);

    // only if q present, use the search
    if query_params.contains_key("q") {
        let datapoints = dataset_handler
            .dataset_controller
            .search_dataset(&dataset_id, &query_params.get("q").unwrap())
            .await
            .map_err(|e| {
                Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
            })?;
        context.insert("results", &datapoints);
    }

    TEMPLATES
        .render("search.html.tera", &context)
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(Html)
}
