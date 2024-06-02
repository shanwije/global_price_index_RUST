use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Deserialize)]
struct BinanceResponse {
    b: String,
    a: String,
}

pub struct BinanceService;

impl BinanceService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_mid_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let (mut socket, _) = connect_async("wss://stream.binance.com:9443/ws/btcusdt@bookTicker").await?;
        if let Some(Ok(Message::Text(text))) = socket.next().await {
            let response: BinanceResponse = serde_json::from_str(&text)?;
            let bid_price: f64 = response.b.parse()?;
            let ask_price: f64 = response.a.parse()?;
            return Ok((bid_price + ask_price) / 2.0);
        }
        Err("Invalid message format".into())
    }
}
