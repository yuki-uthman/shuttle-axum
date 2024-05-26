use crate::helpers::setup_app;
use crate::error::Result;

#[tokio::test]
async fn health() -> Result<()> {
    let app = setup_app().await?;

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
    let app = setup_app().await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/database", app.address()))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {}", e))?;
    assert_eq!(text, "1");

    Ok(())
}

#[tokio::test]
async fn secrets() -> Result<()> {
    let app = setup_app().await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/secrets", app.address()))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {}", e))?;
    assert_eq!(text, "secret value");

    Ok(())
}
