use api::build_router;
use config::{Config as ConfigCrate, File};
use sqlx::{Connection, PgConnection, PgPool, Row};

use crate::error::Result;

const CONFIG_FILE: &str = "dev.yaml";

#[derive(serde::Deserialize, Debug)]
struct Application {
    port: u16,
    host: String,
}

impl Application {
    fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(serde::Deserialize, Debug)]
struct Database {
    username: String,
    password: String,
    host: String,
    port: u16,
    database_name: String,
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    /// Omitting the database name connects to the Postgres instance, not a specific logical database.
    /// This is useful for operations that create or drop databases.
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(serde::Deserialize, Debug)]
struct Config {
    application: Application,
    database: Database,
}

fn get_config() -> Result<Config> {
    let base_dir = std::env::current_dir().unwrap();
    let config_path = base_dir.join(CONFIG_FILE);
    let config = ConfigCrate::builder()
        .add_source(File::with_name(config_path.to_str().unwrap()))
        .build()
        .unwrap();

    Ok(config
        .try_deserialize::<Config>()
        .map_err(|_| "Failed to parse config")?)
}

async fn check_database(config: &Config) -> Result<()> {
    let mut connection = PgConnection::connect(&config.database.connection_string_without_db())
        .await
        .map_err(|_| "Failed to connect to Postgres")?;

    let select_query = "SELECT 1";

    let row = sqlx::query(select_query)
        .fetch_one(&mut connection)
        .await
        .map_err(|_| format!("Failed to execute query: {}", select_query))?;

    let value: i32 = row
        .try_get(0)
        .map_err(|_| "Failed to retrieve query result")?;

    if value != 1 {
        return Err(format!(
            "Query did not return the expected result: {} -> {}",
            select_query, value
        )
        .into());
    }

    Ok(())
}

async fn create_database(config: &Config) -> Result<()> {
    let mut connection = PgConnection::connect(&config.database.connection_string_without_db())
        .await
        .map_err(|_| "Failed to connect to Postgres")?;

    let query_string = format!(r#"CREATE DATABASE "{}";"#, config.database.database_name);

    sqlx::query(&query_string)
        .execute(&mut connection)
        .await
        .map_err(|_| format!("Failed to execute query: {}", query_string))?;

    Ok(())
}

async fn migrate_database(config: &Config) -> Result<()> {
    let mut connection = PgConnection::connect(&config.database.connection_string())
        .await
        .map_err(|_| "Failed to connect to Postgres")?;

    sqlx::migrate!("../../migrations")
        .run(&mut connection)
        .await
        .map_err(|_| "Failed to run migrations")?;

    Ok(())
}

async fn setup_database(config: &Config) -> Result<()> {
    check_database(config).await?;

    create_database(config).await?;

    migrate_database(config).await?;

    Ok(())
}

fn load_secret() -> Result<()> {
    dotenvy::from_path("../../Secrets.toml").map_err(|_| "Failed to load Secrets.toml. Did you forget to change Secrets.toml.example to Secrets.toml?")?;

    Ok(())
}

async fn start_app(config: &Config) -> Result<u16> {
    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .map_err(|_| "Failed to connect to Postgres")?;

    let router = build_router(pool);

    let listener = tokio::net::TcpListener::bind(config.application.address())
        .await
        .unwrap();

    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async {
        axum::serve(listener, router).await.unwrap();
    });


    Ok(port)
}

pub struct App {
    config: Config,
}

impl App {
    pub fn address(&self) -> String {
        self.config.application.address()
    }
}

pub async fn setup_app() -> Result<App> {
    let mut config = get_config()?;

    config.database.database_name = uuid::Uuid::new_v4().to_string();
    setup_database(&config).await?;

    load_secret()?;

    let port = start_app(&config).await?;

    config.application.port = port;


    Ok(App { config })
}
