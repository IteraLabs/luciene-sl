use anchor_lang::prelude::*;

/// Historical price data storage for moving average calculations
#[account]
pub struct DataPrices {
    pub authority: Pubkey,
    pub last_updated: i64,
    pub current_index: u16,
    pub prices: [f32; 10],      // Store last 10 prices for longest MA
    pub timestamps: [i64; 10],  // last 10 timestamps
    pub is_full: bool,          // True when we've filled all 10 slots
    pub bump: u8,
}

impl DataPrices {

    /// Size
    pub const LEN: usize = 8 +  // discriminator
        32 +                    // authority
        8 +                     // last_updated
        2 +                     // current_index
        4 * 10 +                // prices
        8 * 10 +                // timestamps
        1 +                     // is_full
        1;                      // bump

    /// Add new price to circular buffer
    pub fn add_price(&mut self, price: f32, timestamp: i64) {

        self.prices[self.current_index as usize] = price;
        self.timestamps[self.current_index as usize] = timestamp;
        self.last_updated = timestamp;
        
        self.current_index = (self.current_index + 1) % 10;
        
        if self.current_index == 0 && !self.is_full {
            self.is_full = true;
        }

    }

    /// Get recent prices for moving average calculation
    pub fn get_recent_prices(&self, count: usize) -> &[f32] {

        let available_count = if self.is_full { 10 } else { self.current_index as usize };
        let take_count = count.min(available_count);
        &self.prices[..take_count]

    }
}
