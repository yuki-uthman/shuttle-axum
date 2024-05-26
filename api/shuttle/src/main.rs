use api::build_router;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Secrets.toml
    // SECRET_KEY = 'this is a secret key'
    for secret in secrets.into_iter() {
        std::env::set_var(secret.0, secret.1);
    }

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = build_router(pool);

    Ok(router.into())
}
