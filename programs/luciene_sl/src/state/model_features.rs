use anchor_lang::prelude::*;

#[account]
pub struct ModelFeatures {

    pub authority: Pubkey,              // Authorized caller
    pub last_update: i64,               // Time of the latest execution
    pub price_periods: [u32; 5],        // Moving average period
    pub computed_features: [f32; 5],    // Storage of computed features

    pub bump: u8,
}

impl ModelFeatures {

    pub const LEN: usize = 8 +          // discriminator
        32 +                            // authority
        8 +                             // last_update
        4 * 5 +                         // price_periods
        4 * 5 +                         // computed_features
        1;                              // bump

}

