mod constants;
mod errors;
mod events;
mod instructions;
mod states;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("GQKqoMVW3BuSzFRRkfeVsLPArAkRiZkd1vkVNGeqRmJG");

#[program]
pub mod p2p {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        fee_bps: u16,
        fiat_deadline_secs: i64,
        dispute_deadline_secs: i64,
        dispute_fee_escrow: u64,
    ) -> Result<()> {
        ctx.accounts.initialize(
            fee_bps,
            fiat_deadline_secs,
            dispute_deadline_secs,
            dispute_fee_escrow,
            ctx.bumps.global_config,
        )
    }

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        ctx.accounts.create_escrow(amount, &ctx.bumps)
    }

    pub fn mark_escrow_as_paid(ctx: Context<MarkEscrowAsPaid>, escrow_id: u64) -> Result<()> {
        ctx.accounts.mark_escrow_as_paid(escrow_id)
    }

    pub fn release_tokens_in_escrow(
        ctx: Context<ReleaseTokensInEscrow>,
        escrow_id: u64,
        signature: [u8; 64],
    ) -> Result<()> {
        ctx.accounts.release_tokens_in_escrow(escrow_id, signature)
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>, escrow_id: u64) -> Result<()> {
        ctx.accounts.cancel_escrow(escrow_id)
    }

    pub fn create_dispute(ctx: Context<CreateDispute>, escrow_id: u64) -> Result<()> {
        ctx.accounts.create_dispute(escrow_id)
    }

    pub fn resolve_dispute(ctx: Context<ResolveDispute>, escrow_id: u64) -> Result<()> {
        ctx.accounts
            .resolve_dispute(escrow_id, ctx.bumps.dispute_vault)
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>) -> Result<()> {
        ctx.accounts.withdraw_spl()
    }
}
