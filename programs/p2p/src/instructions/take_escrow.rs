use anchor_lang::prelude::*;

use crate::{constants::ESCROW_SEED, errors::P2pError, events, states::Escrow};

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct TakeEscrow<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, escrow_id.to_le_bytes().as_ref()],
        bump = escrow.bump,
        constraint = escrow.buyer == Pubkey::default() @ P2pError::EscrowAlreadyTaken,
    )]
    pub escrow: Account<'info, Escrow>,
}

impl<'info> TakeEscrow<'info> {
    pub fn take_escrow(&mut self, _escrow_id: u64) -> Result<()> {
        self.escrow.buyer = self.taker.key();

        emit!(events::EscrowTaken {
            id: self.escrow.id,
            seller: self.escrow.seller,
            buyer: self.taker.key(),
            mint: self.escrow.mint,
            amount: self.escrow.amount,
        });

        Ok(())
    }
}
