use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{GLOBAL_CONFIG_SEED, MINT_VAULT_SEED},
    errors::P2pError,
    states::{GlobalConfig, MintVault},
};

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = authority
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [MINT_VAULT_SEED, mint.key().as_ref()],
        bump = mint_vault.bump,
        constraint = mint_vault.available_amount > 0 @ P2pError::NoAvailableFundsToWithdraw,
    )]
    pub mint_vault: Account<'info, MintVault>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = mint_vault,
        associated_token::token_program = token_program
    )]
    pub mint_vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
        associated_token::token_program = token_program,
    )]
    pub authority_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawSpl<'info> {
    pub fn withdraw_spl(&mut self) -> Result<()> {
        // transfer tokens to authority ata
        let mint_key = self.mint.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[MINT_VAULT_SEED, mint_key.as_ref(), &[self.mint_vault.bump]]];

        let cpi_accounts = anchor_spl::token::Transfer {
            from: self.mint_vault_ata.to_account_info(),
            to: self.authority_ata.to_account_info(),
            authority: self.mint_vault.to_account_info(),
        };

        let ctx_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        anchor_spl::token::transfer(ctx_cpi, self.mint_vault.available_amount)?;

        // reset available amount to withdraw in mint vault
        self.mint_vault.available_amount = 0;

        Ok(())
    }
}
