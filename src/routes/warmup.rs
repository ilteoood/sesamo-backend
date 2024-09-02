use actix_web::{get, HttpResponse, Responder};

#[get("/warmup")]
async fn handler() -> impl Responder {
    HttpResponse::Ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[test]
    async fn test_warmup_handler() {
        let app = test::init_service(App::new().service(handler)).await;
        let req = test::TestRequest::get().uri("/warmup").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
