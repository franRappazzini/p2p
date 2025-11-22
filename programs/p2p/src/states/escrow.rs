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
    // pub dispute_opened_at: i64,        // Timestamp de apertura de disputa ✅ Necesario
    // pub dispute_opened_by: Pubkey,     // Quién abrió la disputa (seller o buyer) ✅ Necesario pero cambiar por enum o bool
    pub bump: u8,
}

impl Escrow {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + Escrow::INIT_SPACE;

    pub fn can_cancel(&self, fiat_deadline_secs: i64) -> bool {
        if let EscrowState::Open(timestamp) = self.state {
            Clock::get().unwrap().unix_timestamp > timestamp + fiat_deadline_secs
        } else {
            false
        }
    }
}

// data = timestamp
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum EscrowState {
    Open(i64),
    FiatPaid(i64),
    InDispute(i64),
}
