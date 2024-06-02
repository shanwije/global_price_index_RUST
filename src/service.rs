use std::sync::Arc;

use anyhow::Result;
use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;
use log::{debug, error, info};
use tokio::time::{Duration, sleep};

use crate::exchanges::{binance::BinanceService, huobi::HuobiService, kraken::KrakenService};

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

    pub async fn start_collecting_prices(&self) {
        // Immediate price collection on startup
        if let Err(e) = self.collect_and_store_prices().await {
            error!("Initial price collection failed: {:?}", e);
        }

        // Periodic price collection
        let service = self.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = service.collect_and_store_prices().await {
                    error!("Periodic price collection failed: {:?}", e);
                }
                sleep(Duration::from_secs(30)).await;
            }
        });
    }

    async fn collect_and_store_prices(&self) -> Result<()> {
        let mut redis_conn = self.redis_pool.get().await?;

        let binance_price = self.binance_service.get_mid_price().await;
        let kraken_price = self.kraken_service.get_mid_price().await;
        let huobi_price = self.huobi_service.get_mid_price().await;

        if let Ok(price) = binance_price {
            redis_conn.set_ex::<&str, f64, ()>("binance_price", price, 60).await?;
        }
        if let Ok(price) = kraken_price {
            redis_conn.set_ex::<&str, f64, ()>("kraken_price", price, 60).await?;
        }
        if let Ok(price) = huobi_price {
            redis_conn.set_ex::<&str, f64, ()>("huobi_price", price, 60).await?;
        }

        Ok(())
    }

    pub async fn get_average_mid_price(&self) -> Result<f64> {
        let mut redis_conn = self.redis_pool.get().await?;

        let mut prices = Vec::new();
        let keys = vec!["binance_price", "kraken_price", "huobi_price"];
        let mut attempts = 0;

        while attempts < 10 {
            for &key in &keys {
                if let Ok(price) = redis_conn.get::<&str, f64>(key).await {
                    prices.push(price);
                }
            }

            if prices.len() == keys.len() {
                break;
            }

            attempts += 1;
            sleep(Duration::from_millis(100)).await;
        }

        if prices.is_empty() {
            return Err(anyhow::anyhow!("No prices available in cache"));
        }

        let average_price = prices.iter().sum::<f64>() / prices.len() as f64;
        Ok(average_price)
    }
}
