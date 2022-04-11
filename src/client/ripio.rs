use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Ripio {}

#[derive(Debug, Deserialize)]
struct Response {
    last_price: String,
}

#[async_trait]
impl MedianizerClient for Ripio {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let response = reqwest::get(format!(
            "https://api.exchange.ripio.com/api/v1/rate/{}",
            get_pair(pair_code, "_")
        ))
        .await?
        .json::<Response>()
        .await;

        Ok(vec![MedianizerPrice {
            name: "ripio".to_string(),
            price: response.map_or("".to_string(), |response| response.last_price),
            pair_code: pair_code.to_string(),
        }])
    }
}
