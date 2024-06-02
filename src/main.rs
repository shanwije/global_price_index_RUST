use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::info;
use std::sync::Arc;
use crate::config::{Config, create_redis_pool};
use crate::service::AppService;
use crate::controller::init_routes;

mod config;
mod service;
mod controller;
mod exchanges;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Loading configuration from environment");
    let config = Arc::new(Config::from_env().expect("Failed to load config"));
    info!("Configuration loaded successfully");

    info!("Creating Redis connection pool");
    let redis_pool = create_redis_pool(&config);
    info!("Redis connection pool created");

    let app_service = AppService::new(redis_pool.clone(), config.clone());
    info!("Starting price collection service");
    app_service.start_collecting_prices().await;
    info!("Price collection service started");

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
