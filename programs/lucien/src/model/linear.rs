use anchor_lang::prelude::*;
use libm::exp;

pub struct LinearRegression;

impl LinearRegression {
    /// Apply sigmoid function for binary classification
    pub fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + exp(-x as f64) ) as f32
    }

    /// Predict using linear regression weights
    pub fn predict(weights: &[f32; 5], bias: f32, features: &[f32; 5]) -> f32 {
        let mut prediction = 0.0;
        for i in 0..5 {
            prediction += weights[i] * features[i] + bias;
        }
        prediction
    }

    /// Binary classification using logistic regression
    pub fn classify(weights: &[f32; 5], bias: f32, features: &[f32; 5]) -> (u8, f32) {
        let linear_output = Self::predict(weights, bias, features);
        let probability = Self::sigmoid(linear_output) as f32;
        let prediction = if probability > 0.5 { 1 } else { 0 };
        (prediction, probability)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient price history for calculation")]
    InsufficientPriceHistory,
    #[msg("Empty price array provided")]
    EmptyPriceArray,
    #[msg("Mismatched array lengths")]
    MismatchedArrayLengths,
    #[msg("Insufficient data for calculation")]
    InsufficientData,
    #[msg("Empty array provided")]
    EmptyArray,
}

