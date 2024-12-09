use poem::http::StatusCode;
use poem::{handler, Response};

#[handler]
pub async fn home_page() -> Response {
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/dashboard")
        .finish()
}
