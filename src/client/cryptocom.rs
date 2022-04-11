use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::client::common::get_pair;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct CryptoCom {}

#[derive(Debug, Deserialize)]
struct Data {
    k: f64,
}

#[derive(Debug, Deserialize)]
struct ResponseResult {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Response {
    result: ResponseResult,
}

#[async_trait]
impl MedianizerClient for CryptoCom {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let params = [("instrument_name", get_pair(pair_code, "_"))];
        let url = reqwest::Url::parse_with_params(
            "https://api.crypto.com/v2/public/get-ticker",
            &params,
        )?;

        let response = reqwest::get(url).await?.json::<Response>().await;

        Ok(vec![MedianizerPrice {
            name: "cryptocom".to_string(),
            price: response.map_or("".to_string(), |response| {
                response.result.data.k.to_string()
            }),
            pair_code: pair_code.to_string(),
        }])
    }
}
