use anchor_lang::prelude::*;
use crate::{
    InitializeModel,
    InitializeResults,
    InitializeFeatures,
    InitializePriceHistory
};

pub fn initialize_model(
    ctx: Context<InitializeModel>,
    weights: [f32; 5],
    bias: f32,
    ) -> Result<()> {

    let model_params = &mut ctx.accounts.model_params;
    let bump = ctx.bumps.model_params;
    
    model_params.authority = ctx.accounts.authority.key();
    model_params.last_update = Clock::get()?.unix_timestamp;
    model_params.weights = weights;
    model_params.bias = bias;
    model_params.is_active = true;
    model_params.bump = bump;
    
    msg!("Model parameters initialized");
    
    Ok(())
}

pub fn initialize_results(ctx: Context<InitializeResults>) -> Result<()> {

    let model_results = &mut ctx.accounts.model_results;
    let bump = ctx.bumps.model_results;
    
    model_results.authority = ctx.accounts.authority.key();
    model_results.last_update = Clock::get()?.unix_timestamp;
    model_results.latest_prediction = 0;
    model_results.price_at_prediction = 0.0;
    model_results.predictions_count = 0;
    model_results.bump = bump;
    
    msg!("Model results account initialized");
    
    Ok(())

}

pub fn initialize_features(ctx: Context<InitializeFeatures>) -> Result<()> {

    let model_features = &mut ctx.accounts.model_features;
    let bump = ctx.bumps.model_features;
    
    model_features.authority = ctx.accounts.authority.key();
    model_features.last_update = Clock::get()?.unix_timestamp;
    model_features.price_periods = [0; 5];
    model_features.bump = bump;
    
    msg!("Model Features account initialized");
    
    Ok(())

}

pub fn initialize_price_history(ctx: Context<InitializePriceHistory>) -> Result<()> {
    let price_history = &mut ctx.accounts.price_history;
    let bump = ctx.bumps.price_history;
    
    price_history.authority = ctx.accounts.authority.key();
    price_history.last_updated = 0;
    price_history.current_index = 0;
    price_history.prices = [0.0; 10];
    price_history.timestamps = [0; 10];
    price_history.is_full = false;
    price_history.bump = bump;
    
    msg!("Price history account initialized with capacity for 10 price points");
    
    Ok(())
}

