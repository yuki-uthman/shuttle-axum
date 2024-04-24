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
struct Config {
    pub application: Application,
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
