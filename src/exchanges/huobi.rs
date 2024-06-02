use futures_util::{StreamExt, SinkExt};
use log::error;
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use flate2::read::GzDecoder;
use std::io::Read;

#[derive(Deserialize)]
struct HuobiTick {
    bids: Vec<Vec<f64>>,
    asks: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
struct HuobiResponse {
    tick: HuobiTick,
}

pub struct HuobiService;

impl HuobiService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_mid_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let (mut socket, _) = connect_async("wss://api.huobi.pro/ws").await?;
        let subscribe_message = r#"
            {
                "sub": "market.btcusdt.depth.step0",
                "id": "id1"
            }
        "#;
        socket.send(Message::Text(subscribe_message.to_string())).await?;

        while let Some(Ok(message)) = socket.next().await {
            if let Message::Binary(bin_msg) = message {
                let mut d = GzDecoder::new(&bin_msg[..]);
                let mut s = String::new();
                d.read_to_string(&mut s)?;

                if s.contains("ping") {
                    let pong_msg = s.replace("ping", "pong");
                    socket.send(Message::Text(pong_msg)).await?;
                } else if s.contains("tick") {
                    let response: HuobiResponse = serde_json::from_str(&s)?;
                    if let Some(bid) = response.tick.bids.first() {
                        if let Some(ask) = response.tick.asks.first() {
                            let mid_price = (bid[0] + ask[0]) / 2.0;
                            return Ok(mid_price);
                        }
                    }
                } else {
                    error!("Unexpected message format: {}", s);
                }
            }
        }

        Err("Invalid message format".into())
    }
}
