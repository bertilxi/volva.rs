use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Huobi {}

#[derive(Debug, Deserialize)]
struct Tick {
    ask: [f64; 2],
}

#[derive(Debug, Deserialize)]
struct Response {
    tick: Tick,
}

#[async_trait]
impl MedianizerClient for Huobi {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let params = [("symbol", get_pair(pair_code, "").to_lowercase())];
        let url =
            reqwest::Url::parse_with_params("https://api.huobi.pro/market/detail/merged", &params)?;

        let response = reqwest::get(url).await?.json::<Response>().await;

        Ok(vec![MedianizerPrice {
            name: "huobi".to_string(),
            price: response.map_or("".to_string(), |response| response.tick.ask[0].to_string()),
            pair_code: pair_code.to_string(),
        }])
    }
}
