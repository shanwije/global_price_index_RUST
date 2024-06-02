use reqwest::Error;

pub async fn get_kraken_price() -> Result<f64, Error> {
    Ok(70000.0)
}
