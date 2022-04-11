use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Binance {}

#[derive(Debug, Deserialize)]
struct Response {
    price: String,
}

#[async_trait]
impl MedianizerClient for Binance {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let params = [("symbol", get_pair(pair_code, ""))];
        let url = reqwest::Url::parse_with_params(
            "https://api.binance.com/api/v3/ticker/price",
            &params,
        )?;

        let response: Response = reqwest::get(url).await?.json().await?;

        Ok(vec![MedianizerPrice {
            name: "binance".to_string(),
            price: response.price,
            pair_code: pair_code.to_string(),
        }])
    }
}
