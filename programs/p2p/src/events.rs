use anchor_lang::prelude::*;

#[event]
pub struct EscrowCreated {
    pub id: u64,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct MarkEscrowAsPaid {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub marked_at: i64,
}

#[event]
pub struct TokensReleased {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct EscrowCancelled {
    pub id: u64,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub returned_amount: u64,
    pub canceled_at: i64,
}

#[event]
pub struct DisputeCreated {
    pub id: u64,
    pub disputant: Pubkey,
    pub disputed_at: i64,
}

#[event]
pub struct DisputeResolved {
    pub id: u64,
    pub winner: Pubkey,
    pub resolved_at: i64,
}
