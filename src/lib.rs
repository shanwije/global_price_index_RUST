pub mod config;
pub mod controller;
pub mod service;
pub mod exchanges;

use actix_web::web::ServiceConfig;

pub fn init_routes(cfg: &mut ServiceConfig) {
    controller::init_routes(cfg);
}
