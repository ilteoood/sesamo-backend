use std::sync::LazyLock;

use crate::{
    guards::can_open_guard,
    models::{MessageResponse, OpenRequest},
};
use actix_web::{post, web, HttpResponse, Responder};

static OK_MESSAGE: LazyLock<MessageResponse> = LazyLock::new(|| MessageResponse {
    message_id: String::from("test_ok"),
});

fn ok_response() -> HttpResponse {
    HttpResponse::Ok().json(&*OK_MESSAGE)
}

#[post("/{object}")]
async fn handler(
    object: web::Path<String>,
    request_body: web::Json<OpenRequest>,
) -> impl Responder {
    let has_access = can_open_guard(&request_body.into_inner(), &object).await;

    has_access.err().unwrap_or_else(ok_response)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[test]
    async fn test_ok_response() {
        let response = ok_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    async fn invoke_handler(server_id: &str, device_id: &str) -> MessageResponse {
        env::set_var("FIRESTORE_DATABASE", "test");
        let app = test::init_service(App::new().service(handler)).await;
        let req = test::TestRequest::post()
            .uri("/gate")
            .set_json(&OpenRequest {
                server_id: server_id.to_string(),
                device_id: device_id.to_string(),
            })
            .to_request();

        let response = test::call_service(&app, req).await;

        test::read_body_json(response).await
    }

    #[test]
    async fn test_invalid_server_handler() {
        let response = invoke_handler("test", "test").await;
        assert_eq!(response.message_id, "invalid_server");
    }

    #[test]
    async fn test_ok_handler() {
        let response = invoke_handler("test_server", "test_device").await;
        assert_eq!(response, *OK_MESSAGE);
    }
}
