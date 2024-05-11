use actix_web::{get, HttpResponse, Responder};

#[get("/warmup")]
async fn handler() -> impl Responder {
    HttpResponse::Ok()
}
