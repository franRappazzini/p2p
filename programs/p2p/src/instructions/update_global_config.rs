use anchor_lang::prelude::*;

use crate::{constants::GLOBAL_CONFIG_SEED, states::GlobalConfig};

#[derive(Accounts)]
pub struct UpdateGlobalConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = authority ,
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

impl<'info> UpdateGlobalConfig<'info> {
    pub fn update_global_config(
        &mut self,
        authority: Option<Pubkey>,
        fee_bps: Option<u16>,
        fiat_deadline_secs: Option<i64>,
        dispute_deadline_secs: Option<i64>,
        dispute_fee_escrow: Option<u64>,
    ) -> Result<()> {
        if let Some(authority) = authority {
            self.global_config.authority = authority;
        }
        if let Some(fee_bps) = fee_bps {
            self.global_config.fee_bps = fee_bps;
        }
        if let Some(fiat_deadline_secs) = fiat_deadline_secs {
            self.global_config.fiat_deadline_secs = fiat_deadline_secs;
        }
        if let Some(dispute_deadline_secs) = dispute_deadline_secs {
            self.global_config.dispute_deadline_secs = dispute_deadline_secs;
        }
        if let Some(dispute_fee_escrow) = dispute_fee_escrow {
            self.global_config.dispute_fee_escrow = dispute_fee_escrow;
        }

        Ok(())
    }
}
