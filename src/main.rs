use client::lib::get_prices;
use futures::future::join_all;

use crate::client::common::MedianizerResult;

mod client;

#[tokio::main]
async fn main() {
    let pairs = vec![
        "ETH/ARS",
        "BTC/ARS",
        "USDT/ARS",
        "DAI/ARS",
        "USDC/ARS",
        "ETH/BTC",
        "BTC/DAI",
        "BTC/USDT",
        "BTC/USDC",
        "ETH/DAI",
        "ETH/USDT",
        "ETH/USDC",
        "DAI/USDT",
        "DAI/USDC",
        "USDC/USDT",
    ];

    let requests = pairs.iter().map(|pair| get_prices(pair));

    let results: Vec<MedianizerResult> = join_all(requests)
        .await
        .into_iter()
        .map(|result| result.unwrap())
        .filter(|result| !result.prices.is_empty())
        .collect();

    let json = serde_json::to_string(&results).unwrap();

    println!("{}", json);
}
