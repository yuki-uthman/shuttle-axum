use crate::helpers::spawn_app;
use api_lib::Result;

#[tokio::test]
async fn health() -> Result<()> {
    let app = spawn_app().await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", app.address()))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    Ok(())
}

#[tokio::test]
async fn database() -> Result<()> {
    let app = spawn_app().await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/database", app.address()))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    Ok(())
}
