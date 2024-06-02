use std::error::Error;

use reqwest::Client;
use serde_json::Value;

pub struct HuobiService {
    client: Client,
    api_url: String,
}

impl HuobiService {
    pub fn new() -> Self {
        HuobiService {
            client: Client::new(),
            api_url: "https://api.huobi.pro/market/detail/merged?symbol=btcusdt".to_string(),
        }
    }

    pub async fn get_mid_price(&self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        self.calculate_mid_price_snapshot().await
    }

    async fn calculate_mid_price_snapshot(&self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        let response = self.client.get(&self.api_url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        let ask = json["tick"]["ask"][0]
            .as_f64()
            .ok_or("Failed to get ask price")?;
        let bid = json["tick"]["bid"][0]
            .as_f64()
            .ok_or("Failed to get bid price")?;

        let mid_price = (ask + bid) / 2.0;
        Ok(mid_price)
    }
}
