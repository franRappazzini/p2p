use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{ESCROW_SEED, GLOBAL_CONFIG_SEED, MINT_VAULT_SEED},
    states::{Escrow, GlobalConfig, MintVault},
};

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

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
    pub fn create_escrow(&mut self, amount: u64, escrow_bump: u8) -> Result<()> {
        Ok(())
    }
}
