import * as anchor from "@coral-xyz/anchor";

import { DISPUTE_VAULT_SEED, ESCROW_SEED, GLOBAL_CONFIG_SEED, MINT_VAULT_SEED } from "./constants";
import { escrowParser, globalConfigParser } from "./parsers";

import { P2p } from "../../target/types/p2p";
import { bn } from "./functions";

// global config account
async function getGlobalConfigAccount(program: anchor.Program<P2p>) {
  const [globalConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [GLOBAL_CONFIG_SEED],
    program.programId
  );
  return globalConfigParser(await program.account.globalConfig.fetch(globalConfigPda));
}

// escrow accounts
async function getEscrowAccount(program: anchor.Program<P2p>, id: number) {
  const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [ESCROW_SEED, bn(id).toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  return escrowParser(await program.account.escrow.fetch(escrowPda));
}

async function getAllEscrowAccounts(program: anchor.Program<P2p>) {
  return await program.account.escrow.all();
}

// mint vault accounts
async function getMintVaultAccount(program: anchor.Program<P2p>, mint: anchor.web3.PublicKey) {
  const [mintVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [MINT_VAULT_SEED, mint.toBuffer()],
    program.programId
  );
  return await program.account.mintVault.fetch(mintVaultPda);
}

// dispute vault account
async function getDisputeVaultAccount(
  connection: anchor.web3.Connection,
  program: anchor.Program<P2p>
) {
  const [disputeVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [DISPUTE_VAULT_SEED],
    program.programId
  );

  return await connection.getAccountInfo(disputeVaultPda);
}

export {
  getGlobalConfigAccount,
  getEscrowAccount,
  getAllEscrowAccounts,
  getMintVaultAccount,
  getDisputeVaultAccount,
};
