mod config;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use config::Config;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().expect("Failed to load config");

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind((config.server_host.as_str(), config.server_port))?
    .run()
    .await
}
