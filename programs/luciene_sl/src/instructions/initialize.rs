use anchor_lang::prelude::*;
use crate::{
    InitializeParams,
    InitializeResults,
    InitializeFeatures,
    InitializeExperiments,
    InitializeDataPrices
};

pub fn initialize_params(
    ctx: Context<InitializeParams>,
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
    model_features.price_periods = [1; 5];
    model_features.computed_features = [0.1; 5];
    model_features.bump = bump;
    
    msg!("Model Features account initialized");
    
    Ok(())

}

pub fn initialize_experiments(ctx: Context<InitializeExperiments>) -> Result<()> {

    let model_experiments = &mut ctx.accounts.model_experiments;
    let bump = ctx.bumps.model_experiments;
    
    model_experiments.authority = ctx.accounts.authority.key();
    model_experiments.last_update = Clock::get()?.unix_timestamp;
    model_experiments.iterations = 0.0;
    model_experiments.best_loss = 0.0;
    model_experiments.bump = bump;
    
    msg!("Model Experiments account initialized");
    
    Ok(())

}

pub fn initialize_data_prices(ctx: Context<InitializeDataPrices>) -> Result<()> {

    let data_prices = &mut ctx.accounts.data_prices;
    let bump = ctx.bumps.data_prices;
    
    data_prices.authority = ctx.accounts.authority.key();
    data_prices.last_updated = 0;
    data_prices.current_index = 0;
    data_prices.prices = [0.0; 10];
    data_prices.timestamps = [0; 10];
    data_prices.is_full = false;
    data_prices.bump = bump;
    
    msg!("Data Prices account initialized with capacity for 10 price points");
    
    Ok(())

}

