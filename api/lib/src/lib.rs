use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

async fn health() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub fn build_router() -> Router {
    Router::new().route("/", get(health))
}
