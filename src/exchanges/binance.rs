use reqwest::Error;

pub async fn get_binance_price() -> Result<f64, Error> {
    Ok(65000.0)
}
