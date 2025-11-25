import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { bn } from "./functions";

// seeds
const GLOBAL_CONFIG_SEED = Buffer.from("global_config");
const ESCROW_SEED = Buffer.from("escrow");
const MINT_VAULT_SEED = Buffer.from("mint_vault");
const DISPUTE_VAULT_SEED = Buffer.from("dispute_vault");

// initial configs
const FEE_BPS = 100; // 1% fee
// const FIAT_DEADLINE_SECS = bn(1800); // 30 minutes
// const DISPUTE_DEADLINE_SECS = bn(43200); // 12 hours
const DISPUTE_FEE_ESCROW = bn(0.1 * LAMPORTS_PER_SOL); // lamports

// TEST initial configs
const FIAT_DEADLINE_SECS = bn(1); // TEST
const DISPUTE_DEADLINE_SECS = bn(1); // TEST

export {
  FEE_BPS,
  FIAT_DEADLINE_SECS,
  DISPUTE_DEADLINE_SECS,
  DISPUTE_FEE_ESCROW,
  GLOBAL_CONFIG_SEED,
  ESCROW_SEED,
  MINT_VAULT_SEED,
  DISPUTE_VAULT_SEED,
};
