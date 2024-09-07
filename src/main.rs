use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger;
use routes::{open, test, warmup};
use std::env;

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

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/open").service(open::handler))
            .service(web::scope("/test").service(test::handler))
            .service(web::scope("/_ah").service(warmup::handler))
    })
    .bind((BIND_ADDRESS, bind_port))?
    .run()
    .await
}
