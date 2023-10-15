use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct ClientData {
    client_id: String,
    redirect_uris: Vec<String>,
}

pub async fn register_client(json: web::Json<ClientData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
      INSERT INTO clients (id, client_id, client_secret, redirect_uris, created_at)
      VALUES ($1, $2, $3, $4, $5)
              "#,
        Uuid::new_v4(),
        &json.client_id,
        Uuid::new_v4().to_string(),
        &json.redirect_uris,
        Utc::now(),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
