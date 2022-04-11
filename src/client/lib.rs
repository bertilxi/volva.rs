use anyhow::Result;

use futures::future::join_all;

use crate::client::common::{MedianizerClient, MedianizerPrice, MedianizerResult};

use super::{
    binance::Binance, bitso::Bitso, buda::Buda, coinbase::Coinbase, criptoya::CriptoYa,
    cryptocom::CryptoCom, cryptomkt::CryptoMkt, ftx::Ftx, gateio::Gateio, huobi::Huobi,
    kucoin::Kucoin, ripio::Ripio,
};

pub async fn get_prices(pair_code: &str) -> Result<MedianizerResult> {
    let mut prices: Vec<MedianizerPrice> = Vec::new();
    let providers: Vec<Box<dyn MedianizerClient>> = vec![
        // Box::new(Okx {}),
        Box::new(Binance {}),
        Box::new(Bitso {}),
        Box::new(Buda {}),
        Box::new(Coinbase {}),
        Box::new(CriptoYa {}),
        Box::new(CryptoCom {}),
        Box::new(CryptoMkt {}),
        Box::new(Ftx {}),
        Box::new(Gateio {}),
        Box::new(Huobi {}),
        Box::new(Kucoin {}),
        Box::new(Ripio {}),
    ];

    let requests = providers
        .iter()
        .map(|provider| provider.get_price(pair_code));

    let mut pair_prices: Vec<MedianizerPrice> = join_all(requests)
        .await
        .into_iter()
        .flat_map(|price| match price {
            Ok(data) => data,
            Err(_) => vec![MedianizerPrice {
                name: "".to_string(),
                pair_code: "".to_string(),
                price: "".to_string(),
            }],
        })
        .filter(|price| !price.price.is_empty())
        .collect();

    pair_prices.sort_by(|a, b| a.price.cmp(&b.price));

    prices.append(&mut pair_prices);

    if prices.is_empty() {
        return Ok(MedianizerResult {
            pair_code: pair_code.to_string(),
            prices,
            price: MedianizerPrice {
                pair_code: "".to_string(),
                price: "".to_string(),
                name: "".to_string(),
            },
        });
    }

    let middle = (prices.len() - 1) / 2;

    let price = prices[middle].clone();

    Ok(MedianizerResult {
        pair_code: pair_code.to_string(),
        prices,
        price,
    })
}
