use actix_web::{post, web, HttpResponse, Responder};

use crate::models::{MessageResponse, OpenRequest};

#[post("/{name}")]
async fn handler(_name: web::Path<String>, _request_body: web::Json<OpenRequest>) -> impl Responder {
    let response = MessageResponse {
        message_id: "open_ok".to_string(),
    };

    HttpResponse::Ok().json(response)
}