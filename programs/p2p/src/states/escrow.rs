use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR_SIZE;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    id: u64,
    buyer: Pubkey,
    seller: Pubkey,
    mint: Pubkey,
    amount: u64,
    bump: u8,
}

impl Escrow {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + Escrow::INIT_SPACE;
}
