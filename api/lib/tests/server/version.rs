use crate::helpers::spawn_app;

#[tokio::test]
async fn version() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/version", app.address()))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}
