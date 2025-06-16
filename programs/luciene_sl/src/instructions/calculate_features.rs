use anchor_lang::prelude::*;
use crate::CalculateFeatures;
use crate::models::features;
use crate::errors::LucieneError;

pub fn calculate_features(
    ctx: Context<CalculateFeatures>
    ) -> Result<()> {

    let model_features = &mut ctx.accounts.model_features;
    let data_prices = &ctx.accounts.data_prices;
    
    // Get required number of prices for longest moving average
    let max_period = model_features.price_periods.iter().max().unwrap_or(&50);
    
    // Get recent prices
    let recent_prices = data_prices.get_recent_prices(*max_period as usize);
    
    // Initialize moving average calculator
    let ma_calculator = features::MovingAverageCalculator::new(&model_features.price_periods);
    
    // Calculate simple moving averages
    let moving_averages = ma_calculator.calculate_sma(&recent_prices)
        .map_err(|_| LucieneError::FeatureCalculationFailed)?;
    
    // Update model results with calculated features
    model_features.last_update = Clock::get()?.unix_timestamp;
    model_features.computed_features = moving_averages;
    
    msg!("Features values = [{:.6}, {:.6}, {:.6}, {:.6}, {:.6}]",
         moving_averages[0], moving_averages[1], moving_averages[2],
         moving_averages[3], moving_averages[4]);
    
    Ok(())
}
