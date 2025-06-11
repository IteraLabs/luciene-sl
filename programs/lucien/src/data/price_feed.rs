use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};

pub struct PriceDataExtractor;

impl PriceDataExtractor {

    /// Extract price from Pyth price update account
    ///
    /// SOL/USD feed ID on Pyth
    /// 0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d
    ///
    /// USDC/USDC feed ID on Pyth
    /// 0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a
    ///
    pub fn get_oracle_price(
        price_update_account: &PriceUpdateV2,
        feed_id: &str
    ) -> Result<(f64, i64)> {

        let oracle_feed_id = get_feed_id_from_hex(feed_id)
            .map_err(|_| error!(ErrorCode::InvalidFeedId))?;
        
        let price_feed = price_update_account
            .get_price_unchecked(&oracle_feed_id)
            .map_err(|_| error!(ErrorCode::PriceFeedNotFound))?;

        let price = price_feed.price;
        let expo = price_feed.exponent;
        let publish_time = price_feed.publish_time;

        let adjusted_price = price as f64 * 10f64.powi(expo);

        Ok((adjusted_price, publish_time))    

    }

    /// Calculate Base/Quote price
    pub fn calculate_pair_midprice(base_price: f64, quote_price: f64) -> f32 {
        if quote_price != 0.0 {
            base_price as f32 / quote_price as f32
        } else {
            base_price as f32 // Fallback to Base token price if quote price is invalid
        }
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid feed ID provided")]
    InvalidFeedId,
    #[msg("Price feed not found")]
    PriceFeedNotFound,
    #[msg("Invalid price data")]
    InvalidPriceData,
    #[msg("Price confidence too low")]
    LowPriceConfidence,
}


