use anchor_lang::prelude::*;
use crate::errors::*;
use crate::FetchPrice;
use crate::data::price_feed::PriceDataExtractor;

pub fn fetch_and_store_price(ctx: Context<FetchPrice>) -> Result<()> {

    let price_history = &mut ctx.accounts.price_history;
    let price_update = &ctx.accounts.price_update; 
    
    // Extract SOL and USDC prices from Pyth oracle
    let sol_usd = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; 
    let usdc_usd = "0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";

    let (sol_price, sol_timestamp) = PriceDataExtractor::get_oracle_price(price_update, sol_usd)
        .map_err(|_| MLTradingError::PriceFeedNotFound)?;
        
    let (usdc_price, _usdc_timestamp) = PriceDataExtractor::get_oracle_price(price_update, usdc_usd)
        .map_err(|_| MLTradingError::PriceFeedNotFound)?;
    
    // Calculate SOL/USDC midprice
    let midprice = PriceDataExtractor::calculate_pair_midprice(sol_price, usdc_price);
    
    // Store the midprice in price history
    price_history.add_price(midprice, sol_timestamp);
    
    msg!("Price stored: SOL/USDC = {:.6}, SOL/USD = {:.2}, USDC/USD = {:.4}", 
         midprice, sol_price, usdc_price);
    
    Ok(())
}
