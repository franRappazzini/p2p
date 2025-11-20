use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR_SIZE;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    // pub in_dispute: bool, // TODO (fran): implement dispute state to "pause" the escrow
    pub bump: u8,
}

impl Escrow {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + Escrow::INIT_SPACE;
}
