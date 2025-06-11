use anchor_lang::prelude::*;
use crate::{InitializeModel, InitializeResults, InitializePriceHistory};

pub fn initialize_model(
    ctx: Context<InitializeModel>,
    weights: [f32; 5],
    bias: f32,
    ) -> Result<()> {

    let params = &mut ctx.accounts.params;
    let bump = ctx.bumps.params;
    
    params.authority = ctx.accounts.authority.key();
    params.last_update = Clock::get()?.unix_timestamp;
    params.weights = weights;
    params.bias = bias;
    params.is_active = true;
    params.bump = bump;
    
    msg!("Model parameters initialized");
    
    Ok(())
}

pub fn initialize_results(ctx: Context<InitializeResults>) -> Result<()> {

    let results = &mut ctx.accounts.results;
    let bump = ctx.bumps.results;
    
    results.authority = ctx.accounts.authority.key();
    results.last_update = Clock::get()?.unix_timestamp;
    results.latest_prediction = 0;
    results.price_at_prediction = 0.0;
    results.predictions_count = 0;
    results.bump = bump;
    
    msg!("Model results account initialized");
    
    Ok(())

}

pub fn initialize_price_history(ctx: Context<InitializePriceHistory>) -> Result<()> {
    let price = &mut ctx.accounts.price;
    let bump = ctx.bumps.price;
    
    price.authority = ctx.accounts.authority.key();
    price.last_updated = 0;
    price.current_index = 0;
    price.prices = [0.0; 10];
    price.timestamps = [0; 10];
    price.is_full = false;
    price.bump = bump;
    
    msg!("Price history account initialized with capacity for 50 price points");
    
    Ok(())
}

