use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Buda {}

#[derive(Debug, Deserialize)]
struct Ticker {
    last_price: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Response {
    ticker: Ticker,
}

#[async_trait]
impl MedianizerClient for Buda {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let url = format!(
            "https://www.buda.com/api/v2/markets/{}/ticker",
            get_pair(pair_code, "-")
        );

        let response: Response = reqwest::get(url).await?.json().await?;

        let price = response.ticker.last_price[0].clone();

        Ok(vec![MedianizerPrice {
            name: "buda".to_string(),
            price,
            pair_code: pair_code.to_string(),
        }])
    }
}
