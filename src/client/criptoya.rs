use anyhow::Result;

use async_trait::async_trait;
use futures::future::join_all;
use serde::Deserialize;

use super::common::{MedianizerClient, MedianizerPrice};

pub struct CriptoYa {}

#[derive(Debug, Deserialize)]
struct Response {
    ask: f64,
}

#[async_trait]
impl MedianizerClient for CriptoYa {
    async fn get_price(&self, pair_code: &str) -> Result<Vec<MedianizerPrice>> {
        let pair = pair_code.to_lowercase();
        let urls = vec![
            format!("https://criptoya.com/api/latamex/{}", pair),
            format!("https://criptoya.com/api/bitex/{}", pair),
            format!("https://criptoya.com/api/letsbit/{}", pair),
            format!("https://criptoya.com/api/decrypto/{}", pair),
            format!("https://criptoya.com/api/cryptomkt/{}", pair),
            format!("https://criptoya.com/api/bitso/{}", pair),
        ];

        let requests = urls
            .iter()
            .map(|url| async { reqwest::get(url.clone()).await?.json::<Response>().await });

        let responses: Vec<MedianizerPrice> = join_all(requests)
            .await
            .into_iter()
            .map(|response| MedianizerPrice {
                name: "criptoya".to_string(),
                price: response.map_or("".to_string(), |response| response.ask.to_string()),
                pair_code: pair_code.to_string(),
            })
            .collect();

        Ok(responses)
    }
}
