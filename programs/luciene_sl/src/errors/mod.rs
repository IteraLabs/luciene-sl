use anchor_lang::prelude::*;

#[error_code]
pub enum LucieneError {
    #[msg("Model parameters not initialized")]
    ModelNotInitialized,
    
    #[msg("Insufficient price history for calculations")]
    InsufficientPriceHistory,
    
    #[msg("Price data is too old")]
    StalePriceData,
    
    #[msg("Invalid price data received")]
    InvalidPriceData,
    
    #[msg("Model prediction failed")]
    PredictionFailed,
    
    #[msg("Feature calculation failed")]
    FeatureCalculationFailed,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Model is not active")]
    ModelInactive,
    
    #[msg("Price confidence too low")]
    LowPriceConfidence,
    
    #[msg("Mathematical computation error")]
    ComputationError,
    
    #[msg("Array length mismatch")]
    ArrayLengthMismatch,
    
    #[msg("Price feed not found")]
    PriceFeedNotFound,
    
    #[msg("Invalid feed ID")]
    InvalidFeedId,
    
    #[msg("Training data incomplete")]
    IncompleteTrainingData,
    
    #[msg("Metrics update failed")]
    MetricsUpdateFailed,
}
