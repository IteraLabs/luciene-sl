#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(dead_code)]

use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use anchor_lang::prelude::*;

/// OnChain Model
pub mod model;

/// OnChain Instructions
pub mod instructions;

/// OnChain Data
pub mod data;

/// SVM Errors
pub mod errors;

/// To initialize
use crate::model::params::ModelParameters;
use crate::model::results::ModelResults;
use crate::data::price::PriceHistory;

// Add these exports at the top
pub use instructions::initialize;

declare_id!("9QokYaXoDsLBUCnimMZ8nNpB48prsoUkNXtppzu62SAj");

#[program]
pub mod lucien {
    use super::*;

    /// Initialize model parameters account
    pub fn initialize_model(ctx: Context<InitializeModel>, weights: [f32; 5], bias: f32,
    ) -> Result<()> {
        instructions::initialize::initialize_model(ctx, weights, bias)
    }
    
    /// Initialize model results account
    pub fn initialize_results(ctx: Context<InitializeResults>) -> Result<()> {
        instructions::initialize::initialize_results(ctx)
    }
    
    /// Initialize price history account
    pub fn initialize_price_history(ctx: Context<InitializePriceHistory>) -> Result<()> {
        instructions::initialize::initialize_price_history(ctx)
    }

    /// Fetch latest prices from Pyth oracle and update price history
    pub fn fetch_and_store_price(ctx: Context<FetchPrice>) -> Result<()> {
        instructions::fetch_price::fetch_and_store_price(ctx)
    }

}

/// Context for model initialization
#[derive(Accounts)]
pub struct InitializeModel<'info> {
    #[account(
        init,
        payer = authority,
        space = ModelParameters::LEN,
        seeds = [b"params", authority.key().as_ref()],
        bump
    )]
    pub params: Account<'info, ModelParameters>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeResults<'info> {
    #[account(
        init,
        payer = authority,
        space = ModelResults::LEN,
        seeds = [b"results", params.key().as_ref()],
        bump
    )]
    pub results: Account<'info, ModelResults>,
    
    #[account(seeds = [b"params", authority.key().as_ref()], bump)]
    pub params: Account<'info, ModelParameters>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePriceHistory<'info> {
    #[account(
        init,
        payer = authority,
        space = PriceHistory::LEN,
        seeds = [b"price", authority.key().as_ref()],
        bump
    )]
    pub price: Account<'info, PriceHistory>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FetchPrice<'info> {
    #[account(
        mut,
        seeds = [b"price", authority.key().as_ref()],
        bump
    )]

    pub price_history: Account<'info, PriceHistory>,
    
    /// Pyth price update account
    pub price_update: Account<'info, PriceUpdateV2>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RunInference<'info> {
    #[account(
        seeds = [b"params", authority.key().as_ref()],
        bump
    )]
    pub params: Account<'info, ModelParameters>,
    
    #[account(
        mut,
        seeds = [b"results", params.key().as_ref()],
        bump
    )]
    pub results: Account<'info, ModelResults>,
    
    #[account(
        seeds = [b"training_artifacts", params.key().as_ref()],
        bump
    )]
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

