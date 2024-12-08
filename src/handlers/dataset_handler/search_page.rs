use crate::{handlers::dataset_handler::DatasetHandler, TEMPLATES};
use poem::{
    handler,
    web::{Data, Html, Path, Query},
    Error,
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

    let search_query = query_params.get("q");
    // only if q present, use the search
    if let Some(search_query) = search_query {
        let datapoints = dataset_handler
            .dataset_controller
            .search_dataset(&dataset_id, &search_query)
            .await
            .map_err(|e| {
                Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
            })?;
        context.insert("results", &datapoints);
        context.insert("query", &search_query);
    }

    TEMPLATES
        .render("search.html.tera", &context)
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(Html)
}
