use anchor_lang::prelude::*;

#[event]
pub struct EscrowCreated {
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
