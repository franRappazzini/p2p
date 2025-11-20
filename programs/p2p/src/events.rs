use anchor_lang::prelude::*;

#[event]
pub struct EscrowCreated {
    pub id: u64,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct EscrowTaken {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TokensReleased {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
