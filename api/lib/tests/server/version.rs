use crate::helpers::spawn_app;
use api_lib::Result;

#[tokio::test]
async fn version() -> Result<()> {
    let app = spawn_app().await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/version", app.address()))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    Ok(())
}
