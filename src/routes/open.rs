use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    firebase::get_firestore_instance,
    guards::can_open_guard,
    models::{self, firebase::ObjectConfiguration, MessageResponse, OpenRequest},
};

fn ok_response() -> HttpResponse {
    HttpResponse::Ok().json(MessageResponse {
        message_id: "open_ok".to_string(),
    })
}

fn ko_response() -> HttpResponse {
    HttpResponse::InternalServerError().json(MessageResponse {
        message_id: "ko".to_string(),
    })
}

async fn http_post_handler(
    object_configuration: ObjectConfiguration,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    client
        .post(object_configuration.url)
        .form(object_configuration.body.as_str())
        .send()
        .await
}

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
    let object_configuration =
        firebase_instance.get_object_configuration(&request_body.server_id, &object);

    let handler_result = match firebase_server_type {
        models::firebase::ServerDocumentType::HttpPost => http_post_handler(object_configuration),
    };

    if let Ok(result) = handler_result.await {
        if result.status().is_success() {
            return ok_response();
        }
    }

    ko_response()
}
