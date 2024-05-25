use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::PgPool;

mod error;

pub use error::{Error, Result};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

async fn health() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn database(
    State(state): State<AppState>,
) -> std::result::Result<impl IntoResponse, impl IntoResponse> {
    let result: std::result::Result<i32, sqlx::Error> = sqlx::query_scalar("SELECT 1")
        .fetch_one(&state.pool)
        .await;

    match result {
        Ok(result) => Ok((StatusCode::OK, result.to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn build_router(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/health", get(health))
        .route("/database", get(database))
        .with_state(state)
}
