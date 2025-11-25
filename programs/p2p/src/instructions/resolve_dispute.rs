use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{DISPUTE_VAULT_SEED, ESCROW_SEED, GLOBAL_CONFIG_SEED, MINT_VAULT_SEED},
    states::{Escrow, EscrowState, GlobalConfig, MintVault},
};

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct ResolveDispute<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub to: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = authority,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [DISPUTE_VAULT_SEED],
        bump,
    )]
    pub dispute_vault: SystemAccount<'info>,

    #[account(
        mut,
        close = to,
        seeds = [ESCROW_SEED, escrow_id.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = mint,
        constraint = matches!(escrow.state, EscrowState::ReDispute(_)),
        constraint = to.key() == escrow.buyer || to.key() == escrow.seller,
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
        associated_token::authority = to,
        associated_token::token_program = token_program,
    )]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = mint_vault,
        associated_token::token_program = token_program,
    )]
    pub mint_vault_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveDispute<'info> {
    pub fn resolve_dispute(&mut self, _escrow_id: u64, dispute_vault_bump: u8) -> Result<()> {
        // transfer SOL from dispute_vault to 'to' account
        let signer_seeds: &[&[&[u8]]] = &[&[DISPUTE_VAULT_SEED, &[dispute_vault_bump]]];

        let cpi_accounts = system_program::Transfer {
            from: self.dispute_vault.to_account_info(),
            to: self.to.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        system_program::transfer(cpi_ctx, self.global_config.dispute_fee_escrow)?;

        // transfer tokens from escrow
        let mint_key = self.mint.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[MINT_VAULT_SEED, mint_key.as_ref(), &[self.mint_vault.bump]]];

        let cpi_accounts = anchor_spl::token::Transfer {
            from: self.mint_vault_ata.to_account_info(),
            to: self.to_ata.to_account_info(),
            authority: self.mint_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        let fee = self.global_config.calculate_fee(self.escrow.amount);

        let amount = if self.to.key() == self.escrow.buyer {
            // complete release to buyer (amount - fee)
            self.escrow.amount.checked_sub(fee).unwrap()
        } else {
            // refund to seller (full deposit amount)
            self.escrow.amount.checked_add(fee).unwrap()
        };

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        // update available lamports in global_config
        self.global_config.add_available_lamports();

        // update available amount in mint_vault if flow is completed (to buyer)
        if self.to.key() == self.escrow.buyer {
            self.mint_vault.add_available_amount(fee);
        }

        Ok(())
    }
}
