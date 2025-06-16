use anchor_lang::prelude::*;

pub struct MovingAverageCalculator<'a> {
    pub periods: &'a [u32; 5],
}

impl<'a> MovingAverageCalculator<'a> {

    pub fn new(periods: &'a [u32; 5]) -> Self {
        Self { periods }
    }

    pub fn calculate_sma(&self, prices: &[f32]) -> Result<[f32; 5]> {

        let mut smas = [0.0; 5];
        
        for (i, &period) in self.periods.iter().enumerate() {
            let sum: f32 = prices.iter().take(period as usize).sum();
            smas[i] = sum / (period as f32);
        }
        
        Ok(smas)
    }
}

