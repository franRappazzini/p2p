use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{ESCROW_SEED, GLOBAL_CONFIG_SEED, MINT_VAULT_SEED},
    events,
    states::{Escrow, EscrowDisputedBy, EscrowState, GlobalConfig, MintVault},
};

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub buyer: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer = creator,
        space = Escrow::SIZE,
        seeds = [ESCROW_SEED, global_config.escrow_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = creator,
        space = MintVault::SIZE,
        seeds = [MINT_VAULT_SEED, mint.key().as_ref()],
        bump,
    )]
    pub mint_vault: Account<'info, MintVault>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = creator,
        associated_token::token_program = token_program,
    )]
    pub creator_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = mint_vault,
        associated_token::token_program = token_program,
    )]
    pub mint_vault_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateEscrow<'info> {
    pub fn create_escrow(&mut self, amount: u64, bumps: &CreateEscrowBumps) -> Result<()> {
        // tranfer tokens to mint vault ata
        let cpi_account = anchor_spl::token::Transfer {
            from: self.creator_ata.to_account_info(),
            to: self.mint_vault_ata.to_account_info(),
            authority: self.creator.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_account);

        let fee = self.global_config.calculate_fee(amount);

        anchor_spl::token::transfer(cpi_ctx, amount.checked_add(fee).unwrap())?;

        // set escrow data
        self.escrow.set_inner(Escrow {
            id: self.global_config.escrow_count,
            seller: self.creator.key(),
            buyer: self.buyer.key(),
            mint: self.mint.key(),
            amount,
            state: EscrowState::Open(Clock::get()?.unix_timestamp),
            disputed_by: EscrowDisputedBy::Nobody,
            bump: bumps.escrow,
        });

        // increment escrow counter
        self.global_config.increment_escrow_count();

        // set mint vault data if not already set
        if !self.mint_vault.is_initialized {
            self.mint_vault.set_inner(MintVault {
                is_initialized: true,
                mint: self.mint.key(),
                available_amount: 0, // will be updated on release, not here
                bump: bumps.mint_vault,
            });
        }

        // emit event
        emit!(events::EscrowCreated {
            id: self.escrow.id,
            seller: self.creator.key(),
            mint: self.mint.key(),
            amount,
        });

        Ok(())
    }
}
