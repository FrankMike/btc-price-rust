use reqwest;
use tokio;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    symbol: String,
    price: String,
}

#[tokio::main]
async fn main() {
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT".to_string();
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let p: Price = serde_json::from_str(&response).unwrap();
    let value = p.price.parse::<f64>().unwrap();
    println!("Bitcoin: $ {}", value);
}
