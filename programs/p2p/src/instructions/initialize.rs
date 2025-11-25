use anchor_lang::{prelude::*, system_program};

use crate::{
    constants::{DISPUTE_VAULT_SEED, GLOBAL_CONFIG_SEED},
    states::GlobalConfig,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = GlobalConfig::SIZE,
        seeds = [GLOBAL_CONFIG_SEED],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [DISPUTE_VAULT_SEED],
        bump,
    )]
    pub dispute_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        fee_bps: u16,
        fiat_deadline_secs: i64,
        dispute_deadline_secs: i64,
        dispute_fee_escrow: u64,
        global_config_bump: u8,
    ) -> Result<()> {
        self.global_config.set_inner(GlobalConfig {
            authority: self.authority.key(),
            escrow_count: 0,
            fee_bps,
            fiat_deadline_secs,
            dispute_deadline_secs,
            dispute_fee_escrow,
            available_lamports: 0,
            bump: global_config_bump,
        });

        // create dispute vault account
        let rent = Rent::get()?.minimum_balance(self.dispute_vault.data_len());

        let cpi_accounts = system_program::Transfer {
            from: self.authority.to_account_info(),
            to: self.dispute_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi_ctx, rent)
    }
}
