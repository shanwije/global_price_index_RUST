use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;
use crate::exchanges::{get_binance_price, get_kraken_price, get_huobi_price};

pub async fn get_global_price_index(redis_pool: &Pool) -> Result<f64, Box<dyn std::error::Error>> {
    let mut redis_conn = redis_pool.get().await?;

    let cache_key = "global_price_index";
    if let Ok(price) = redis_conn.get::<_, f64>(cache_key).await {
        return Ok(price);
    }

    let binance_price = get_binance_price().await?;
    let kraken_price = get_kraken_price().await?;
    let huobi_price = get_huobi_price().await?;

    let average_price = (binance_price + kraken_price + huobi_price) / 3.0;

    redis_conn.set_ex(cache_key, average_price, 60).await?;

    Ok(average_price)
}
