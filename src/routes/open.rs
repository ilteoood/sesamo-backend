use std::sync::LazyLock;

use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use futures::try_join;
use itertools::Itertools;

use crate::{
    firebase::get_firestore_instance,
    guards::can_open_guard,
    models::{self, firebase::ObjectConfiguration, MessageResponse, OpenRequest},
};

static OK_MESSAGE: LazyLock<MessageResponse> = LazyLock::new(|| MessageResponse {
    message_id: String::from("open_ok"),
});

static KO_MESSAGE: LazyLock<MessageResponse> = LazyLock::new(|| MessageResponse {
    message_id: String::from("ko"),
});

fn ok_response() -> HttpResponse {
    HttpResponse::Ok().json(&*OK_MESSAGE)
}

fn ko_response() -> HttpResponse {
    HttpResponse::InternalServerError().json(&*KO_MESSAGE)
}

async fn http_post_handler(
    object_configuration: ObjectConfiguration,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let body: Vec<(&str, &str)> = object_configuration
        .body
        .split("&")
        .map(|x| x.split("=").collect_tuple().unwrap_or(("", "")))
        .collect();

    client
        .post(object_configuration.url)
        .form(&body)
        .send()
        .await
}

#[post("/{object}")]
async fn handler(
    object: web::Path<String>,
    request_body: web::Form<OpenRequest>,
) -> impl Responder {
    let request_body = request_body.into_inner();

    let has_access = can_open_guard(&request_body, &object).await;

    if has_access.is_err() {
        return HttpResponse::InternalServerError().json(has_access.err().unwrap());
    }

    let firebase_instance = get_firestore_instance().await;

    let (firebase_server_type, object_configuration) = try_join!(
        firebase_instance.get_server_type(&request_body.server_id),
        firebase_instance.get_object_configuration(&request_body.server_id, &object)
    )
    .unwrap();

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

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[tokio_shared_rt::test(shared)]
    async fn test_ok_response() {
        let response = ok_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_ko_response() {
        let response = ko_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    async fn invoke_handler(server_id: &str, device_id: &str, object: &str) -> MessageResponse {
        env::set_var("FIRESTORE_DATABASE", "test");
        let app = test::init_service(App::new().service(handler)).await;
        let req = test::TestRequest::post()
            .uri(format!("/{}", object).as_str())
            .set_form(&OpenRequest {
                server_id: server_id.to_string(),
                device_id: device_id.to_string(),
            })
            .to_request();

        let response = test::call_service(&app, req).await;

        test::read_body_json(response).await
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_ok_handler() {
        let response = invoke_handler("test_server", "test_device", "gate").await;
        assert_eq!(response, *OK_MESSAGE);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_ko_handler() {
        let response = invoke_handler("test_server", "test_device", "wicket").await;
        assert_eq!(response, *KO_MESSAGE);
    }
}
