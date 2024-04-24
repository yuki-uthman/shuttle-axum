use crate::helpers::spawn_app;

#[tokio::test]
async fn health() {
    spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000")
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
}
