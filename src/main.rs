use actix_web::{web, App, HttpServer};
use global_price_index::{config::Config, controller::init_routes, service::AppService};
use log::{info, error};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = match Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {:?}", e);
            std::process::exit(1);
        }
    };

    let redis_pool = global_price_index::config::create_redis_pool(&config);
    let app_service = AppService::new(redis_pool.clone());

    tokio::spawn(app_service.clone().start_collecting_prices());

    info!("Starting server at {}:{}", config.server_host, config.server_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_service.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
            .configure(init_routes)
    })
        .bind((config.server_host.as_str(), config.server_port))?
        .run()
        .await
}
