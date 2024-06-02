use reqwest::Error;

pub async fn get_huobi_price() -> Result<f64, Error> {
    Ok(50000.0)
}
