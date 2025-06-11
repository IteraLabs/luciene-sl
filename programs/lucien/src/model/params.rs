use anchor_lang::prelude::*;

#[account]
pub struct ModelParameters {

    pub authority: Pubkey,
    pub last_update: i64,
    
    pub weights: [f32; 5],
    pub bias: f32,

    pub is_active: bool,
    pub bump: u8,
}

impl ModelParameters {

    pub const LEN: usize = 8 +  // discriminator
        32 +                    // authority
        8 +                     // last_update
        4 * 5 +                 // weights
        4 +                     // bias
        1 +                     // is_active
        1;                      // bump

    pub fn predict(&self, features: &[f32; 5]) -> Result<f64> {

        let mut prediction = self.bias;

        for i in 0..self.weights.len() {
            prediction += self.weights[i] * features[i];
        }

        Ok(prediction as f64)

    }

    pub fn classify(&self, features: &[f32; 5]) -> Result<u8> {

        let prediction = self.predict(features)?;
        let sigmoid = 1.0 / (1.0 + (-prediction).exp());
        Ok(if sigmoid > 0.5 { 1 } else { 0 })
    
    }
}

