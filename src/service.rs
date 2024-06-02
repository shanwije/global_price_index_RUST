use crate::exchanges::{binance::BinanceService, kraken::KrakenService, huobi::HuobiService};
use deadpool_redis::Pool;
use log::{error, info};
use deadpool_redis::redis::AsyncCommands;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use anyhow::Result;
use crate::config::Config;

const BINANCE_PRICE_KEY: &str = "binance_price";
const KRAKEN_PRICE_KEY: &str = "kraken_price";
const HUOBI_PRICE_KEY: &str = "huobi_price";

#[derive(Clone)]
pub struct AppService {
    binance_service: Arc<BinanceService>,
    kraken_service: Arc<KrakenService>,
    huobi_service: Arc<HuobiService>,
    redis_pool: Pool,
    config: Arc<Config>,
}

impl AppService {
    pub fn new(redis_pool: Pool, config: Arc<Config>) -> Self {
        Self {
            binance_service: Arc::new(BinanceService::new(&config.binance_api_url)),
            kraken_service: Arc::new(KrakenService::new(&config.kraken_api_url)),
            huobi_service: Arc::new(HuobiService::new(&config.huobi_api_url)),
            redis_pool,
            config,
        }
    }

    pub async fn start_collecting_prices(&self) {
        // Immediate price collection on startup
        if let Err(e) = self.collect_and_store_prices().await {
            error!("Initial price collection failed: {:?}", e);
        }

        // Periodic price collection
        let service = self.clone();
        let interval = self.config.price_collection_interval;
        tokio::spawn(async move {
            loop {
                if let Err(e) = service.collect_and_store_prices().await {
                    error!("Periodic price collection failed: {:?}", e);
                }
                sleep(Duration::from_millis(interval)).await;
            }
        });
    }

    async fn collect_and_store_prices(&self) -> Result<()> {
        info!("Starting to collect and store prices.");

        let mut redis_conn = match self.redis_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to get Redis connection: {:?}", e);
                return Err(e.into());
            }
        };

        info!("Fetching prices from Binance, Kraken, and Huobi.");

        let binance_price = self.binance_service.get_mid_price().await;
        let kraken_price = self.kraken_service.get_mid_price().await;
        let huobi_price = self.huobi_service.get_mid_price().await;

        if let Ok(price) = binance_price {
            info!("Fetched Binance price: {}", price);
            match redis_conn.set_ex::<&str, f64, ()>(BINANCE_PRICE_KEY, price, self.config.cache_expiration_time).await {
                Ok(_) => info!("Successfully stored Binance price in Redis."),
                Err(e) => error!("Failed to store Binance price in Redis: {:?}", e),
            }
        } else {
            error!("Failed to fetch Binance price: {:?}", binance_price);
        }

        if let Ok(price) = kraken_price {
            info!("Fetched Kraken price: {}", price);
            match redis_conn.set_ex::<&str, f64, ()>(KRAKEN_PRICE_KEY, price, self.config.cache_expiration_time).await {
                Ok(_) => info!("Successfully stored Kraken price in Redis."),
                Err(e) => error!("Failed to store Kraken price in Redis: {:?}", e),
            }
        } else {
            error!("Failed to fetch Kraken price: {:?}", kraken_price);
        }

        if let Ok(price) = huobi_price {
            info!("Fetched Huobi price: {}", price);
            match redis_conn.set_ex::<&str, f64, ()>(HUOBI_PRICE_KEY, price, self.config.cache_expiration_time).await {
                Ok(_) => info!("Successfully stored Huobi price in Redis."),
                Err(e) => error!("Failed to store Huobi price in Redis: {:?}", e),
            }
        } else {
            error!("Failed to fetch Huobi price: {:?}", huobi_price);
        }

        info!("Finished collecting and storing prices.");
        Ok(())
    }
    pub async fn get_average_mid_price(&self) -> Result<f64> {
        let mut redis_conn = self.redis_pool.get().await?;

        let mut prices = Vec::new();
        let keys = vec![BINANCE_PRICE_KEY, KRAKEN_PRICE_KEY, HUOBI_PRICE_KEY];
        let mut attempts = 0;

        while attempts < self.config.max_cache_check_attempts {
            for &key in &keys {
                if let Ok(price) = redis_conn.get::<&str, f64>(key).await {
                    prices.push(price);
                }
            }

            if prices.len() == keys.len() {
                break;
            }

            attempts += 1;
            sleep(Duration::from_millis(self.config.cache_check_interval_ms)).await;
        }

        if prices.is_empty() {
            return Err(anyhow::anyhow!("No prices available in cache"));
        }

        let average_price = prices.iter().sum::<f64>() / prices.len() as f64;
        let rounded_average_price = (average_price * 10000.0).round() / 10000.0;
        Ok(rounded_average_price)
    }
}
