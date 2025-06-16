use anchor_lang::prelude::*;
use crate::errors::LucieneError;
use crate::models::linear;
use crate::ModelInference;

pub fn run_inference(ctx: Context<ModelInference>) -> Result<()> {

    let model_params = &ctx.accounts.model_params;
    let model_results = &mut ctx.accounts.model_results;
    let model_features = &mut ctx.accounts.model_features;
    
    // Check if model is active
    if !model_params.is_active {
        return Err(LucieneError::ModelInactive.into());
    }
    
    // Get features from model results (should be calculated in previous step)
    let raw_features = model_features.computed_features;
    
    // Run linear regression prediction
    let (prediction, confidence) = linear::LinearRegression::classify(
        &model_params.weights,
        model_params.bias,
        &raw_features
    );
    
    // Update model results with prediction
    let price_at_pred = model_results.price_at_prediction.clone();

    model_results.update_prediction(
        prediction,
        price_at_pred,
    );
    
    // Log prediction details
    let direction = if prediction == 1 { "UP" } else { "DOWN" };
    msg!("ML Prediction: {} (confidence: {:.3}) at price {:.6}",
         direction, confidence, model_results.price_at_prediction);
    
    Ok(())
}
