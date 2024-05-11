use actix_web::{get, web, Responder};

#[get("/{name}")]
async fn handler(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}