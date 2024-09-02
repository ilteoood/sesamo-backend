use crate::models::MessageResponse;
use actix_web::{post, HttpResponse, Responder};

#[post("/{name}")]
async fn handler() -> impl Responder {
    let response = MessageResponse {
        message_id: "test_ok".to_string(),
    };

    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[test]
    async fn test_test_handler() {
        let app = test::init_service(App::new().service(handler)).await;
        let req = test::TestRequest::post().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let message_response: MessageResponse = test::read_body_json(resp).await;
        assert_eq!(message_response.message_id, "test_ok");
    }
}
