use crate::{
    guards::can_open_guard,
    models::{MessageResponse, OpenRequest},
};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/{object}")]
async fn handler(
    object: web::Path<String>,
    request_body: web::Json<OpenRequest>,
) -> impl Responder {
    let has_access = can_open_guard(&request_body.into_inner(), &object).await;

    if has_access.is_err() {
        return has_access.err().unwrap();
    }

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
