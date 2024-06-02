use actix_web::{web, Responder, get, HttpResponse};
use serde::Serialize;
use crate::service::get_global_price_index;
use log::error;

#[derive(Serialize)]
struct AverageMidPriceDto {
    average_mid_price: f64,
}

#[get("/global-price-index")]
async fn global_price_index(redis_pool: web::Data<deadpool_redis::Pool>) -> impl Responder {
    match get_global_price_index(&redis_pool).await {
        Ok(price) => HttpResponse::Ok().json(AverageMidPriceDto { average_mid_price: price }),
        Err(e) => {
            error!("Failed to get global price index: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(global_price_index);
    cfg.service(health_check);
}
