use actix_web::{get, HttpResponse, Responder, web};
use log::{error, info};
use serde::Serialize;

use crate::service::AppService;

#[derive(Serialize)]
struct AverageMidPriceDto {
    average_mid_price: f64,
}

#[get("/global-price-index")]
async fn global_price_index(service: web::Data<AppService>) -> impl Responder {
    info!("Handling request for /global-price-index");

    match service.get_average_mid_price().await {
        Ok(price) => HttpResponse::Ok().json(AverageMidPriceDto { average_mid_price: price }),
        Err(e) => {
            error!("Failed to get global price index: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to get global price index: {:?}", e))
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(global_price_index);
}
