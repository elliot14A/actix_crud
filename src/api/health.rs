use actix_web::{self, get, HttpResponse};

#[get("/health")]
pub async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}