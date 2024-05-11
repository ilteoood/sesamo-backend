use actix_web::{post, web, HttpResponse, Responder};

use crate::models::{MessageResponse, OpenRequest};

#[post("/{name}")]
async fn handler(name: web::Path<String>, request_body: web::JsonBody<OpenRequest>) -> impl Responder {
    let response = MessageResponse {
        message_id: "open_ok".to_string(),
    };

    HttpResponse::Ok().json(response)
}