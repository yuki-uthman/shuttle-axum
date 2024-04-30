use crate::helpers::spawn_app;

#[tokio::test]
async fn health() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", app.address()))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
}
