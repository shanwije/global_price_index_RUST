use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct KrakenService {
    client: Client,
    api_url: String,
}

impl KrakenService {
    pub fn new(api_url: &str) -> Self {
        KrakenService {
            client: Client::new(),
            api_url: api_url.to_string(),
        }
    }

    pub async fn get_mid_price(&self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        self.calculate_mid_price_snapshot().await
    }

    async fn calculate_mid_price_snapshot(&self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        let response = self.client.get(&self.api_url).send().await?;
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        let ask = json["result"]["XXBTZUSD"]["a"][0]
            .as_str()
            .ok_or("Failed to get ask price")?
            .parse::<f64>()?;
        let bid = json["result"]["XXBTZUSD"]["b"][0]
            .as_str()
            .ok_or("Failed to get bid price")?
            .parse::<f64>()?;

        let mid_price = (ask + bid) / 2.0;
        Ok(mid_price)
    }
}
