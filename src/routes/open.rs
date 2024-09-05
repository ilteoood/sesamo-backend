use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    guards::can_open_guard,
    models::{MessageResponse, OpenRequest},
};

#[post("/{object}")]
async fn handler(
    object: web::Path<String>,
    request_body: web::Json<OpenRequest>,
) -> impl Responder {
    let has_access = can_open_guard(request_body.into_inner(), object).await;

    if has_access.is_err() {
        return has_access.err().unwrap();
    }

    let response = MessageResponse {
        message_id: "open_ok".to_string(),
    };

    HttpResponse::Ok().json(response)
}
