use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR_SIZE;

#[account]
#[derive(InitSpace)]
pub struct MintVault {
    pub mint: Pubkey,
    pub available_amount: u64, // available amount to withdraw
    pub is_initialized: bool,
    pub bump: u8,
}

impl MintVault {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + MintVault::INIT_SPACE;
}
