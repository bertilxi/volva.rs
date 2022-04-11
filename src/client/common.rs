use anyhow::Result;
use serde::{Deserialize, Serialize};

use async_trait::async_trait;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MedianizerPrice {
    pub pair_code: String,
    pub price: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedianizerResult {
    pub pair_code: String,
    pub prices: Vec<MedianizerPrice>,
    pub price: MedianizerPrice,
}

pub fn get_pair(pair_code: &str, separator: &str) -> String {
    let assets = pair_code.split('/').collect::<Vec<&str>>();
    let asset1 = assets[0];
    let asset2 = assets[1];

    format!("{asset1}{separator}{asset2}")
}

#[async_trait]
pub trait MedianizerClient {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>>;
}
