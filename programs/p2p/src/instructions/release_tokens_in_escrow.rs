use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{ESCROW_SEED, GLOBAL_CONFIG_SEED, MINT_VAULT_SEED},
    errors::P2pError,
    events,
    states::{Escrow, GlobalConfig, MintVault},
};

// TODO check TODO.todo
#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct ReleaseTokensInEscrow<'info> {
    #[account(mut)]
    pub buyer: SystemAccount<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        seeds = [GLOBAL_CONFIG_SEED],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        close = seller,
        seeds = [ESCROW_SEED, escrow_id.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = seller,
        has_one = buyer,
        has_one = mint,
        constraint = escrow.can_release() @ P2pError::InvalidEscrowState,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(address = escrow.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [MINT_VAULT_SEED, mint.key().as_ref()],
        bump = mint_vault.bump,
    )]
    pub mint_vault: Account<'info, MintVault>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = mint_vault,
        associated_token::token_program = token_program,
    )]
    pub mint_vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ReleaseTokensInEscrow<'info> {
    pub fn release_tokens_in_escrow(&mut self, _escrow_id: u64) -> Result<()> {
        // transfer tokens to buyer ata
        let mint_key = self.mint.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[MINT_VAULT_SEED, mint_key.as_ref(), &[self.mint_vault.bump]]];

        let cpi_accounts = anchor_spl::token::Transfer {
            from: self.mint_vault_ata.to_account_info(),
            to: self.buyer_ata.to_account_info(),
            authority: self.mint_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        let fee = self.global_config.calculate_fee(self.escrow.amount);

        anchor_spl::token::transfer(cpi_ctx, self.escrow.amount.checked_sub(fee).unwrap())?;

        // update available amount to withdraw in mint vault
        self.mint_vault.add_available_amount(fee);

        // emit event
        emit!(events::TokensReleased {
            id: self.escrow.id,
            seller: self.seller.key(),
            buyer: self.buyer.key(),
            mint: self.mint.key(),
            amount: self.escrow.amount,
        });

        Ok(())
    }
}
