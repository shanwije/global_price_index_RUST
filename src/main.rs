mod config;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use config::Config;
use log::info;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::from_env().expect("Failed to load configuration");

    info!("Starting server at {}:{}", config.server_host, config.server_port);

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind((config.server_host.as_str(), config.server_port))?
    .run()
    .await
}
