use crate::exchanges::{binance::BinanceService, kraken::KrakenService, huobi::HuobiService};
use deadpool_redis::Pool;
use log::{error, info, debug};
use deadpool_redis::redis::AsyncCommands;
use std::sync::Arc;
#[derive(Clone)]
pub struct AppService {
    binance_service: Arc<BinanceService>,
    kraken_service: Arc<KrakenService>,
    huobi_service: Arc<HuobiService>,
    redis_pool: Pool,
}

impl AppService {
    pub fn new(redis_pool: Pool) -> Self {
        Self {
            binance_service: Arc::new(BinanceService::new()),
            kraken_service: Arc::new(KrakenService::new()),
            huobi_service: Arc::new(HuobiService::new()),
            redis_pool,
        }
    }

    pub async fn start_collecting_prices(self) {
        let binance_service = self.binance_service.clone();
        let kraken_service = self.kraken_service.clone();
        let huobi_service = self.huobi_service.clone();
        let redis_pool = self.redis_pool.clone();

        tokio::spawn(async move {
            loop {
                let mut prices = vec![];

                match binance_service.get_mid_price().await {
                    Ok(price) => {
                        debug!("Binance price: {}", price);
                        prices.push(price);
                    },
                    Err(err) => error!("Binance service failed: {:?}", err),
                }

                match kraken_service.get_mid_price().await {
                    Ok(price) => {
                        debug!("Kraken price: {}", price);
                        prices.push(price);
                    },
                    Err(err) => error!("Kraken service failed: {:?}", err),
                }

                match huobi_service.get_mid_price().await {
                    Ok(price) => {
                        debug!("Huobi price: {}", price);
                        prices.push(price);
                    },
                    Err(err) => error!("Huobi service failed: {:?}", err),
                }

                if !prices.is_empty() {
                    let valid_prices: Vec<f64> = prices.into_iter().filter_map(|price| Some(price)).collect();
                    info!("prices : {:?}", valid_prices);
                    let average_price: f64 = valid_prices.iter().sum::<f64>() / valid_prices.len() as f64;

                    let mut redis_conn = redis_pool.get().await.unwrap();
                    redis_conn.set_ex::<&str, f64, ()>("global_price_index", average_price, 1).await.unwrap();
                    info!("Updated global price index: {}", average_price);
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
        });
    }

    pub async fn get_average_mid_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let mut redis_conn = self.redis_pool.get().await?;
        let cache_key = "global_price_index";

        match redis_conn.get::<&str, f64>(cache_key).await {
            Ok(price) => {
                info!("Cache hit for global price index");
                Ok(price)
            },
            Err(e) => {
                error!("Failed to get price from cache: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}