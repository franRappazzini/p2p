import { bn } from "./functions";

// seeds
const GLOBAL_CONFIG_SEED = Buffer.from("global_config");
const ESCROW_SEED = Buffer.from("escrow");
const MINT_VAULT_SEED = Buffer.from("mint_vault");

// initial configs
const FEE_BPS = 100; // 1% fee
const FIAT_DEADLINE_SECS = bn(1800); // 30 minutes

export { FEE_BPS, FIAT_DEADLINE_SECS, GLOBAL_CONFIG_SEED, ESCROW_SEED, MINT_VAULT_SEED };
