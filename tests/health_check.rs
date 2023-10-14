use std::{fmt::format, net::TcpListener};

#[tokio::test]
async fn health_check_success() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failled to bind address");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to assign port");
    let port = listener.local_addr().unwrap().port();
    let server = rust_oauth::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
