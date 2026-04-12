use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        service: "disona-auth".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
