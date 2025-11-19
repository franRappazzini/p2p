use anchor_lang::constant;

pub const DISCRIMINATOR_SIZE: usize = 8;

#[constant]
pub const GLOBAL_CONFIG_SEED: &[u8] = b"global_config";

#[constant]
pub const ESCROW_SEED: &[u8] = b"escrow";

#[constant]
pub const MINT_VAULT_SEED: &[u8] = b"mint_vault";
