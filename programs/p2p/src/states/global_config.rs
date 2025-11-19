use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR_SIZE;

#[account]
#[derive(InitSpace)]
pub struct GlobalConfig {
    pub authority: Pubkey,
    pub escrow_count: u64,
    pub fee_bps: u16,
    pub bump: u8,
}

impl GlobalConfig {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + GlobalConfig::INIT_SPACE;

    pub fn calculate_fee(&self, amount: u64) -> u64 {
        amount
            .checked_mul(self.fee_bps as u64)
            .unwrap()
            .checked_div(10_000)
            .unwrap()
    }

    pub fn increment_escrow_count(&mut self) {
        self.escrow_count = self.escrow_count.checked_add(1).unwrap();
    }

    pub fn decrement_escrow_count(&mut self) {
        self.escrow_count = self.escrow_count.checked_sub(1).unwrap();
    }
}
