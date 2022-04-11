use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct Gateio {}

#[derive(Debug, Deserialize)]
struct Response {
    lowest_ask: String,
}

#[async_trait]
impl MedianizerClient for Gateio {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let params = [("currency_pair", get_pair(pair_code, "_"))];
        let url =
            reqwest::Url::parse_with_params("https://api.gateio.ws/api/v4/spot/tickers", &params)?;

        let response = reqwest::get(url).await?.json::<Vec<Response>>().await;

        Ok(vec![MedianizerPrice {
            name: "gateio".to_string(),
            price: response.map_or("".to_string(), |response| {
                response[0].lowest_ask.to_string()
            }),
            pair_code: pair_code.to_string(),
        }])
    }
}
