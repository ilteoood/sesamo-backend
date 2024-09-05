use actix_web::{
    post,
    web::{self, Path},
    HttpResponse, Responder,
};

use crate::{
    firebase::get_firestore_instance,
    guards::can_open_guard,
    models::{self, MessageResponse, OpenRequest},
};

fn ok_response() -> HttpResponse {
    HttpResponse::Ok().json(MessageResponse {
        message_id: "open_ok".to_string(),
    })
}

fn http_post_handler(object: Path<String>) {}

fn ifttt_handler(object: Path<String>) {}

#[post("/{object}")]
async fn handler(
    object: web::Path<String>,
    request_body: web::Json<OpenRequest>,
) -> impl Responder {
    let request_body = request_body.into_inner();

    let has_access = can_open_guard(&request_body, &object).await;

    if has_access.is_err() {
        return has_access.err().unwrap();
    }

    let firebase_instance = get_firestore_instance().await;
    let firebase_server_type = firebase_instance.get_server_type(&request_body.server_id);

    match firebase_server_type {
        models::firebase::ServerDocumentType::HttpPost => {
            http_post_handler(object);
        }
        models::firebase::ServerDocumentType::IFTTT => {}
    }

    ok_response()
}
