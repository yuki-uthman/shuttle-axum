use api_lib::build_router;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Secrets.toml
    // SECRET_KEY = 'this is a secret key'
    let secret = secrets.get("SECRET_KEY").expect("SECRET_KEY must be set");
    println!("Secret: {}", secret);

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = build_router(pool);

    Ok(router.into())
}
