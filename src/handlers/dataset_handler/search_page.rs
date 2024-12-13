use crate::{
    handlers::dataset_handler::DatasetHandler, models::datapoint::DatapointChunkView, TEMPLATES,
};
use poem::{
    handler,
    web::{Data, Html, Path, Query},
    Error,
};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tera::Context;

#[handler]
pub async fn search_page(
    Data(dataset_handler): Data<&Arc<DatasetHandler>>,
    Path(dataset_id): Path<String>,
    Query(query_params): Query<HashMap<String, String>>,
) -> Result<Html<String>, Error> {
    let mut context = Context::new();
    context.insert("dataset_id", &dataset_id);
    let dataset = dataset_handler
        .dataset_controller
        .get_dataset(&dataset_id)
        .await
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;
    context.insert("dataset_name", &dataset.name);

    let search_query = query_params.get("q");
    // only if q present, use the search
    if let Some(search_query) = search_query {
        let start = Instant::now();
        let datapoints = dataset_handler
            .dataset_controller
            .search_dataset(&dataset_id, &search_query)
            .await
            .map_err(|e| {
                Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
            })?;
        let elapsed = start.elapsed().as_millis();
        let mut results: Vec<DatapointChunkView> = Vec::new();
        let (chunks, datapoints) = datapoints;
        for chunk in chunks {
            let datapoint = datapoints
                .iter()
                .find(|dp| dp.id == chunk.datapoint_id)
                .unwrap();
            results.push(DatapointChunkView::from((chunk, datapoint.clone())));
        }
        context.insert("results", &results);
        context.insert("query", &search_query);
        context.insert("search_duration", &elapsed);
    }

    TEMPLATES
        .render("search.html.tera", &context)
        .map_err(|e| {
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(Html)
}
