import * as anchor from "@coral-xyz/anchor";

import { ESCROW_SEED, GLOBAL_CONFIG_SEED } from "./constants";

import { P2p } from "../../target/types/p2p";
import { bn } from "./functions";

async function getGlobalConfigAccount(program: anchor.Program<P2p>) {
  const [globalConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [GLOBAL_CONFIG_SEED],
    program.programId
  );
  return await program.account.globalConfig.fetch(globalConfigPda);
}

async function getEscrowAccount(program: anchor.Program<P2p>, id: number) {
  const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [ESCROW_SEED, Buffer.from(bn(id).toArray())],
    program.programId
  );
  return await program.account.escrow.fetch(escrowPda);
}

export { getGlobalConfigAccount, getEscrowAccount };
