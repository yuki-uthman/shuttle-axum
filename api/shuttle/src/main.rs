use api_lib::build_router;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let router = build_router(pool);

    Ok(router.into())
}
