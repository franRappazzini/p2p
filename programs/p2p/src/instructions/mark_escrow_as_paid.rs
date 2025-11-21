use anchor_lang::prelude::*;

use crate::{
    constants::{ESCROW_SEED, GLOBAL_CONFIG_SEED},
    errors::P2pError,
    events,
    states::{Escrow, EscrowState, GlobalConfig},
};

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct MarkEscrowAsPaid<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, escrow_id.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = buyer,
        constraint = escrow.state == EscrowState::Open @ P2pError::EscrowAlreadyTaken,
    )]
    pub escrow: Account<'info, Escrow>,
}

impl<'info> MarkEscrowAsPaid<'info> {
    pub fn mark_escrow_as_paid(&mut self, _escrow_id: u64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        self.escrow.state = EscrowState::FiatPaid;
        self.escrow.fiat_paid_at = now;

        emit!(events::MarkEscrowAsPaid {
            id: self.escrow.id,
            seller: self.escrow.seller,
            buyer: self.buyer.key(),
            marked_at: now,
        });

        Ok(())
    }
}
