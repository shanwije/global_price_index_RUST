use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Deserialize)]
struct KrakenResponse {
    #[serde(rename = "b")]
    bid: String,
    #[serde(rename = "a")]
    ask: String,
}

pub struct KrakenService;


impl KrakenService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_mid_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let (mut socket, _) = connect_async("wss://ws.kraken.com").await?;
        let subscribe_message = r#"
            {
                "event": "subscribe",
                "pair": ["XBT/USD"],
                "subscription": {
                    "name": "book"
                }
            }
        "#;
        socket.send(Message::Text(subscribe_message.to_string())).await?;
        if let Some(Ok(Message::Text(text))) = socket.next().await {
            let response: Vec<KrakenResponse> = serde_json::from_str(&text)?;
            let bid_price: f64 = response[0].bid.parse()?;
            let ask_price: f64 = response[0].ask.parse()?;
            return Ok((bid_price + ask_price) / 2.0);
        }
        Err("Invalid message format".into())
    }
}
