use rust_oauth::configuration::{get_configuration, DatabaseSettings};
use rust_oauth::startup::run;
use serde_json::json;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

#[tokio::test]
async fn health_check_success() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failled to bind address");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn clients_post_returns_201_with_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = json!({
        "client_id": "test_client_id",
        "redirect_uris": ["https://test-api.com"]
    });

    // Act
    let body_string = body.to_string();
    let response = client
        .post(&format!("{}/clients", &app.address))
        .header("Content-Type", "application/json")
        .body(body_string)
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT client_id, redirect_uris FROM clients")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch");
    assert_eq!(saved.client_id, "test_client_id");
    // assert_eq!(redirect_uris, ["https:"]);
}

#[tokio::test]
async fn clients_post_returns_400_with_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = vec![
        (
            json!({
                "redirect_uris": ["https://test-api.com"]
            }),
            "missing client_id",
        ),
        (
            json!({
                "client_id": "test_client_id",
            }),
            "missing redirect_uris",
        ),
        (json!({}), "missing client_id and redirect_uris"),
    ];

    // Act
    for (input, error_msg) in body {
        let input_str = input.to_string();
        let response = client
            .post(&format!("{}/clients", &app.address))
            .header("Content-Type", "application/json")
            .body(input_str)
            .send()
            .await
            .expect("Failed to send request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not respond with 400 when {}",
            error_msg
        );
    }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
