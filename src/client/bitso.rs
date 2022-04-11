use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Bitso {}

#[derive(Debug, Deserialize)]
struct Payload {
    ask: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    payload: Payload,
}

#[async_trait]
impl MedianizerClient for Bitso {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let params = [("book", get_pair(pair_code, "_").to_lowercase())];
        let url = reqwest::Url::parse_with_params("https://api.bitso.com/v3/ticker", &params)?;

        let response: Response = reqwest::get(url).await?.json().await?;

        Ok(vec![MedianizerPrice {
            name: "bitso".to_string(),
            price: response.payload.ask,
            pair_code: pair_code.to_string(),
        }])
    }
}
