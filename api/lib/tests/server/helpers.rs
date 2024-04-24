use api_lib::build_router;

pub async fn spawn_app() {
    let app = build_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
    });
}
