use actix_web::{web, Responder, get, HttpResponse};
use serde::Serialize;
use crate::service::get_global_price_index;

#[derive(Serialize)]
struct AverageMidPriceDto {
    average_mid_price: f64,
}

#[get("/global-price-index")]
async fn global_price_index(redis_pool: web::Data<deadpool_redis::Pool>) -> impl Responder {
    match get_global_price_index(&redis_pool).await {
        Ok(price) => HttpResponse::Ok().json(AverageMidPriceDto { average_mid_price: price }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(global_price_index);
}
