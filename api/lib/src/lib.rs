use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

async fn health() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn version(State(state): State<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(&state.pool)
        .await;

    println!("{:#?}", result);

    match result {
        Ok(version) => Ok((StatusCode::OK, version)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn build_router(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/health", get(health))
        .route("/version", get(version))
        .with_state(state)
}
