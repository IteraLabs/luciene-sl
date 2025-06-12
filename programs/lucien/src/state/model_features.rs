use anchor_lang::prelude::*;

#[account]
pub struct ModelFeatures {

    pub authority: Pubkey,
    pub last_update: i64,
    
    // Moving average periods
    pub price_periods: [u32; 5],
    pub computed_features: [f32; 5],

    pub bump: u8,
}

impl ModelFeatures {

    pub const LEN: usize = 8 +  // discriminator
        32 +                    // authority
        8 +                     // last_update
        4 * 5 +                 // price_periods
        4 * 5 +                 // price_periods
        1;                      // bump

}

