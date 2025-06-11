use anchor_lang::prelude::*;
use crate::errors::*;
use crate::model::linear;
use crate::RunInference;

pub fn run_inference(ctx: Context<RunInference>) -> Result<()> {

    let params = &ctx.accounts.params;
    let results = &mut ctx.accounts.results;
    
    // Check if model is active
    if !params.is_active {
        return Err(MLTradingError::ModelInactive.into());
    }
    
    // Get features from model results (should be calculated in previous step)
    let raw_features = results.features_used;
    
    // Run linear regression prediction
    let (prediction, confidence) = linear::LinearRegression::classify(
        &params.weights,
        params.bias,
        &raw_features
    );
    
    // Update model results with prediction
    let price_at_pred = results.price_at_prediction.clone();

    results.update_prediction(
        prediction,
        price_at_pred,
    );
    
    // Log prediction details
    let direction = if prediction == 1 { "UP" } else { "DOWN" };

    msg!("ML Prediction: {} (confidence: {:.3}) at price {:.6}",
         direction, confidence, results.price_at_prediction);
    
    Ok(())
}
