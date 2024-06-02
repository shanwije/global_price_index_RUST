use futures_util::{SinkExt, StreamExt};
use log::{error, debug};
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Deserialize)]
struct BookSnapshot {
    #[serde(rename = "as")]
    asks: Option<Vec<[String; 3]>>,
    #[serde(rename = "bs")]
    bids: Option<Vec<[String; 3]>>,
}

#[derive(Deserialize)]
struct BookUpdate {
    #[serde(rename = "a")]
    asks: Option<Vec<[String; 3]>>,
    #[serde(rename = "b")]
    bids: Option<Vec<[String; 3]>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum KrakenResponse {
    SubscriptionStatus {
        event: String,
        status: String,
        subscription: Option<Subscription>,
    },
    Book(Vec<serde_json::Value>),
}

#[derive(Deserialize)]
struct Subscription {
    name: String,
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
                    "name": "book",
                    "depth": 10
                }
            }
        "#;
        socket.send(Message::Text(subscribe_message.to_string())).await?;

        while let Some(Ok(message)) = socket.next().await {
            if let Message::Text(text) = message {
                match serde_json::from_str::<KrakenResponse>(&text) {
                    Ok(KrakenResponse::Book(data)) => {
                        if let Some(book_data) = data.get(1) {
                            if let Ok(snapshot) = serde_json::from_value::<BookSnapshot>(book_data.clone()) {
                                return self.calculate_mid_price_snapshot(snapshot);
                            } else if let Ok(update) = serde_json::from_value::<BookUpdate>(book_data.clone()) {
                                return self.calculate_mid_price_update(update);
                            }
                        }
                    },
                    Ok(_) => {
                        debug!("Received non-book data: {}", text);
                    },
                    Err(e) => {
                        error!("Error parsing Kraken response: {}", e);
                    }
                }
            }
        }

        Err("Invalid message format".into())
    }

    fn calculate_mid_price_snapshot(&self, data: BookSnapshot) -> Result<f64, Box<dyn std::error::Error>> {
        let bids = data.bids.unwrap_or_default();
        let asks = data.asks.unwrap_or_default();

        self.calculate_mid_price(bids, asks)
    }

    fn calculate_mid_price_update(&self, data: BookUpdate) -> Result<f64, Box<dyn std::error::Error>> {
        let bids = data.bids.unwrap_or_default();
        let asks = data.asks.unwrap_or_default();

        self.calculate_mid_price(bids, asks)
    }

    fn calculate_mid_price(&self, bids: Vec<[String; 3]>, asks: Vec<[String; 3]>) -> Result<f64, Box<dyn std::error::Error>> {
        if asks.is_empty() {
            return Err("Invalid ask data".into());
        }

        let lowest_ask = asks[0][0].parse::<f64>()?;
        let highest_bid = if !bids.is_empty() {
            Some(bids[0][0].parse::<f64>()?)
        } else {
            None
        };

        debug!("Highest bid: {:?}, Lowest ask: {}", highest_bid, lowest_ask);

        if highest_bid.is_some() && highest_bid.unwrap().is_nan() {
            return Err("Invalid bid price".into());
        }

        let mid_price = if let Some(high_bid) = highest_bid {
            (high_bid + lowest_ask) / 2.0
        } else {
            lowest_ask
        };

        debug!("Calculated mid price: {}", mid_price);
        Ok(mid_price)
    }
}
