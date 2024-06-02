use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;
use log::{info, error};
use crate::exchanges::{get_binance_price, get_kraken_price, get_huobi_price};
use tokio_retry::strategy::{ExponentialBackoff, jitter};
use tokio_retry::Retry;

pub async fn get_global_price_index(redis_pool: &Pool) -> Result<f64, Box<dyn std::error::Error>> {
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let mut redis_conn = Retry::spawn(retry_strategy, || async {
        redis_pool.get().await.map_err(|e| {
            error!("Failed to get Redis connection: {:?}", e);
            e
        })
    }).await?;

    let cache_key = "global_price_index";
    if let Ok(price) = redis_conn.get(cache_key).await {
        info!("Cache hit: {}", price);
        return Ok(price);
    }

    let binance_price = get_binance_price().await?;
    let kraken_price = get_kraken_price().await?;
    let huobi_price = get_huobi_price().await?;

    let average_price = (binance_price + kraken_price + huobi_price) / 3.0;
    info!("Calculated average price: {}", average_price);

    redis_conn.set_ex(cache_key, average_price, 60).await.map_err(|e| {
        error!("Failed to set cache: {:?}", e);
        e
    })?;

    Ok(average_price)
}
