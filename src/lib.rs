use actix_web::web::ServiceConfig;

pub mod config;
pub mod controller;
pub mod service;
pub mod exchanges;

pub fn init_routes(cfg: &mut ServiceConfig) {
    controller::init_routes(cfg);
}
