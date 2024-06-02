mod config;
mod controller;
mod service;
mod exchanges;

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
    let redis_pool = config::create_redis_pool(&config);

    info!("Starting server at {}:{}", config.server_host, config.server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .route("/health", web::get().to(health_check))
            .configure(controller::init_routes)
    })
    .bind((config.server_host.as_str(), config.server_port))?
    .run()
    .await
}
