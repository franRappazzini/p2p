use anchor_lang::{prelude::*, system_program};

use crate::{
    constants::{DISPUTE_VAULT_SEED, ESCROW_SEED, GLOBAL_CONFIG_SEED},
    states::{Escrow, GlobalConfig},
};

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct CreateDispute<'info> {
    #[account(mut)]
    pub disputant: Signer<'info>,

    #[account(
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, escrow_id.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        seeds = [DISPUTE_VAULT_SEED],
        bump,
    )]
    pub dispute_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateDispute<'info> {
    pub fn create_dispute(&mut self, _escrow_id: u64) -> Result<()> {
        // update escrow state (checks inside)
        self.escrow.dispute(
            self.global_config.dispute_deadline_secs,
            self.disputant.key(),
        )?;

        // deposit fee escrow
        let cpi_accounts = system_program::Transfer {
            from: self.disputant.to_account_info(),
            to: self.dispute_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi_ctx, self.global_config.dispute_fee_escrow)
    }
}
