use anchor_lang::prelude::*;

#[account]
pub struct ModelResults {

    pub authority: Pubkey,
    pub last_update: i64,
    
    // Latest prediction details
    pub latest_prediction: u8,
    pub price_at_prediction: f32,
    
    // Prediction tracking
    pub predictions_count: u32,
    
    pub bump: u8,
}

impl ModelResults {

    pub const LEN: usize = 8 +  // discriminator
        32 +                    // authority
        8 +                     // last_update
        1 +                     // latest_prediction
        4 +                     // price_at_prediction
        8 +                     // predictions_count
        1;                      // bump

    pub fn update_prediction(
        &mut self,
        prediction: u8,
        price: f32
    ) {
        
        self.latest_prediction = prediction;
        self.price_at_prediction = price;
        self.predictions_count += 1;
        self.last_update = Clock::get().unwrap().unix_timestamp;
        
    }
}
