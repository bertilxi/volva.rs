use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Ftx {}

#[derive(Debug, Deserialize)]
struct ResponseResult {
    price: f64,
}

#[derive(Debug, Deserialize)]
struct Response {
    result: ResponseResult,
}

#[async_trait]
impl MedianizerClient for Ftx {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let url = format!("https://ftx.com/api/markets/{}", pair_code);

        let response = reqwest::get(url).await?.json::<Response>().await;

        Ok(vec![MedianizerPrice {
            name: "ftx".to_string(),
            price: response.map_or("".to_string(), |data| data.result.price.to_string()),
            pair_code: pair_code.to_string(),
        }])
    }
}
