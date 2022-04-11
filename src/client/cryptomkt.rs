use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct CryptoMkt {}

#[derive(Debug, Deserialize)]
struct Response {
    last: String,
}

#[async_trait]
impl MedianizerClient for CryptoMkt {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let pair = get_pair(pair_code, "");
        let url = format!(
            "https://api.exchange.cryptomkt.com/api/3/public/ticker?symbols[]={}",
            pair
        );

        let response = reqwest::get(url)
            .await?
            .json::<HashMap<String, Response>>()
            .await;

        Ok(vec![MedianizerPrice {
            name: "cryptomkt".to_string(),
            price: response
                .unwrap_or_default()
                .get(&pair)
                .map_or("".to_string(), |response| response.last.to_string()),
            pair_code: pair_code.to_string(),
        }])
    }
}
