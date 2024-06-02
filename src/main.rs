use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::info;
use crate::config::{Config, create_redis_pool};
use crate::service::AppService;
use crate::controller::init_routes;
use std::sync::Arc;

mod config;
mod service;
mod controller;
mod exchanges;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Arc::new(Config::from_env().expect("Failed to load config"));
    let redis_pool = create_redis_pool(&config);

    let app_service = AppService::new(redis_pool.clone(), config.clone());
    app_service.start_collecting_prices().await;

    info!("Starting server at {}:{}", config.server_host, config.server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_service.clone()))
            .configure(init_routes)
    })
        .bind((config.server_host.as_str(), config.server_port))?
        .run()
        .await
}
