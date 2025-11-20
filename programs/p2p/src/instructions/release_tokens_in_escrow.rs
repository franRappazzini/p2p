use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenInterface, TokenAccount}};

use crate::{
    constants::{ESCROW_SEED, GLOBAL_CONFIG_SEED, MINT_VAULT_SEED}, errors::P2pError, states::{Escrow, GlobalConfig, MintVault}
};

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct ReleaseTokensInEscrow<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

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
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,
    
    pub associated_token_program:Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ReleaseTokensInEscrow<'info> {
    pub fn release_tokens_in_escrow(&mut self, _escrow_id: u64, signature: [u8; 64]) -> Result<()> {
        // verify signature
        let message = format!("approve_release:{}", self.escrow.key());
        brine_ed25519::sig_verify(&self.seller.key().to_bytes(), &signature, &message.as_bytes())
            .map_err(|err| {
                msg!("Signature verification failed {:?}", err);
                P2pError::SignatureVerificationFailed
            })?;

        // calculate fee

        // transfer tokens to buyer ata

        // update available amount to withdraw in mint vault

        // emit event

        Ok(())
    }
}
