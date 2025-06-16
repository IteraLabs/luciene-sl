use anchor_lang::prelude::*;

#[account]
pub struct ModelExperiments {

    pub authority: Pubkey,
    pub last_update: i64,

    pub iterations: f32,
    pub best_loss: f32,

    pub bump: u8,
}

impl ModelExperiments {

    pub const LEN: usize = 8 +  // discriminator
        32 +                    // authority
        8 +                     // last_update
        4 +                     // iterations
        4 +                     // best_loss
        1;                      // bump

}

