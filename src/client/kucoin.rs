use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Kucoin {}

#[derive(Debug, Deserialize)]
struct Data {
    price: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[async_trait]
impl MedianizerClient for Kucoin {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let params = [("symbol", get_pair(pair_code, "-"))];
        let url = reqwest::Url::parse_with_params(
            "https://api.kucoin.com/api/v1/market/orderbook/level1",
            &params,
        )?;

        let response = reqwest::get(url).await?.json::<Response>().await;

        Ok(vec![MedianizerPrice {
            name: "kucoin".to_string(),
            price: response.map_or("".to_string(), |response| response.data.price),
            pair_code: pair_code.to_string(),
        }])
    }
}
