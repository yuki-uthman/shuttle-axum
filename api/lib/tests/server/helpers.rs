use api_lib::build_router;
use config::{Config as ConfigCrate, File};

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
    pub application: Application,
    pub database: Database,
}

fn get_config() -> Config {
    let base_dir = std::env::current_dir().unwrap();
    let config_path = base_dir.join(CONFIG_FILE);
    let config = ConfigCrate::builder()
        .add_source(File::with_name(config_path.to_str().unwrap()))
        .build()
        .unwrap();

    config.try_deserialize::<Config>().unwrap()
}

pub async fn spawn_app() {
    let config = get_config();

    let app = build_router();

    let listener = tokio::net::TcpListener::bind(config.application.address())
        .await
        .unwrap();

    tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
    });
}
