use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

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
        has_one = mint,
        constraint = matches!(escrow.state, EscrowState::Open(_)) @ P2pError::EscrowAlreadyTaken,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(address = escrow.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> MarkEscrowAsPaid<'info> {
    pub fn mark_escrow_as_paid(&mut self, _escrow_id: u64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        self.escrow.state = EscrowState::FiatPaid(now);

        emit!(events::MarkEscrowAsPaid {
            id: self.escrow.id,
            seller: self.escrow.seller,
            buyer: self.buyer.key(),
            marked_at: now,
        });

        Ok(())
    }
}
