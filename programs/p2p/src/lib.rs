mod constants;
mod errors;
mod instructions;
mod states;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("GQKqoMVW3BuSzFRRkfeVsLPArAkRiZkd1vkVNGeqRmJG");

#[program]
pub mod p2p {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
        ctx.accounts.initialize(fee_bps, ctx.bumps.global_config)
    }

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        ctx.accounts.create_escrow(amount, ctx.bumps.escrow)
    }
}
