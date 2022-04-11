use anyhow::Result;

use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Coinbase {}

#[derive(Debug, Deserialize)]
struct Response {
    price: String,
}

#[async_trait]
impl MedianizerClient for Coinbase {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let url = format!(
            "https://api.exchange.coinbase.com/products/{}/ticker",
            get_pair(pair_code, "-")
        );

        let response = reqwest::Client::new()
            .get(url.clone())
            .header(reqwest::header::USER_AGENT, "volva")
            .send()
            .await?
            .json::<Response>()
            .await;

        Ok(vec![MedianizerPrice {
            name: "coinbase".to_string(),
            price: response.map_or("".to_string(), |response| response.price),
            pair_code: pair_code.to_string(),
        }])
    }
}
