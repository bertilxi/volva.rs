use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Okx {}

#[derive(Debug, Deserialize)]
struct Data {
    last: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Vec<Data>,
}

#[async_trait]
impl MedianizerClient for Okx {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let params = [("instId", get_pair(pair_code, "-"))];
        let url =
            reqwest::Url::parse_with_params("https://www.okx.com/api/v5/market/ticker", &params)?;

        let response = reqwest::get(url).await?.json::<Response>().await;

        Ok(vec![MedianizerPrice {
            name: "okx".to_string(),
            price: response.map_or("".to_string(), |response| response.data[0].last.to_string()),
            pair_code: pair_code.to_string(),
        }])
    }
}
