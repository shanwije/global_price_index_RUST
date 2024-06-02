use deadpool_redis::{Pool, Runtime};
use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub redis_url: String,
    pub binance_api_url: String,
    pub kraken_api_url: String,
    pub huobi_api_url: String,
    pub price_collection_interval: u64,
    pub cache_expiration_time: usize,
    pub cache_check_interval_ms: u64,
    pub max_cache_check_attempts: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        Ok(Self {
            server_host: env::var("SERVER_HOST")?,
            server_port: env::var("SERVER_PORT")?.parse().unwrap(),
            redis_url: env::var("REDIS_URL")?,
            binance_api_url: env::var("BINANCE_API_URL")?,
            kraken_api_url: env::var("KRAKEN_API_URL")?,
            huobi_api_url: env::var("HUOBI_API_URL")?,
            price_collection_interval: env::var("PRICE_COLLECTION_INTERVAL")?.parse().unwrap(),
            cache_expiration_time: env::var("CACHE_EXPIRATION_TIME")?.parse().unwrap(),
            cache_check_interval_ms: env::var("CACHE_CHECK_INTERVAL_MS")?.parse().unwrap(),
            max_cache_check_attempts: env::var("MAX_CACHE_CHECK_ATTEMPTS")?.parse().unwrap(),
        })
    }
}

pub fn create_redis_pool(config: &Config) -> Pool {
    let pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(Runtime::Tokio1))
        .expect("Cannot create Redis pool");
    pool
}
