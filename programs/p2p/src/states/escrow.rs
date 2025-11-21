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
    pub state: EscrowState,
    pub created_at: i64,
    pub fiat_paid_at: i64,
    // pub dispute_opened_at: i64,        // Timestamp de apertura de disputa ✅ Necesario
    // pub dispute_opened_by: Pubkey,     // Quién abrió la disputa (seller o buyer) ✅ Necesario pero cambiar por enum o bool
    pub bump: u8,
}

impl Escrow {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + Escrow::INIT_SPACE;

    pub fn can_cancel(&self, fiat_deadline_secs: i64) -> bool {
        Clock::get().unwrap().unix_timestamp > self.created_at + fiat_deadline_secs
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum EscrowState {
    Open,
    FiatPaid,
    InDispute,
}
