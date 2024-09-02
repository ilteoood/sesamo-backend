use guards::can_open;
use routes::{open, test, warmup};
use std::env;

use actix_web::{guard, web, App, HttpServer};

mod firebase;
mod guards;
mod models;
mod routes;

const BIND_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "3000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_port = env::var("PORT")
        .unwrap_or(DEFAULT_PORT.to_string())
        .parse()
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/open")
                    .guard(guard::fn_guard(can_open))
                    .service(open::handler),
            )
            .service(
                web::scope("/test")
                    .guard(guard::fn_guard(can_open))
                    .service(test::handler),
            )
            .service(web::scope("/_ah").service(warmup::handler))
    })
    .bind((BIND_ADDRESS, bind_port))?
    .run()
    .await
}
