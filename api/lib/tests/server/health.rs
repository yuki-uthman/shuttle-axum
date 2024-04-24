use api_lib::build_router;
use tokio::time::Duration;

#[tokio::test]
async fn health() {
    let app = build_router();

    let (close_tx, close_rx) = tokio::sync::oneshot::channel();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    let server_handle = tokio::spawn(async {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                _ = close_rx.await;
            })
            .await
            .unwrap();
    });

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8000")
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    println!("waiting 5 seconds");
    tokio::time::sleep(Duration::from_secs(5)).await;

    println!("telling server to shutdown");
    _ = close_tx.send(());

    println!("waiting for server to gracefully shutdown");
    _ = server_handle.await;
}
