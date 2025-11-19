import * as anchor from "@coral-xyz/anchor";

import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { getEscrowAccount, getGlobalConfigAccount } from "./utils/accounts";

import { FEE_BPS } from "./utils/constants";
import { P2p } from "../target/types/p2p";
import { Program } from "@coral-xyz/anchor";
import { bn } from "./utils/functions";

describe("p2p", () => {
  const provider = anchor.AnchorProvider.env();
  const { connection, wallet } = provider;

  anchor.setProvider(provider);

  const program = anchor.workspace.p2p as Program<P2p>;

  let randomMint: anchor.web3.PublicKey;

  before(async () => {
    randomMint = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6 // like USDC
    );

    const walletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      randomMint,
      wallet.publicKey
    );

    await mintTo(
      connection,
      wallet.payer,
      randomMint,
      walletAta.address,
      wallet.payer,
      1_000_000_000 // 1,000
    );
  });

  it("`initialize`!", async () => {
    const tx = await program.methods.initialize(FEE_BPS).rpc();
    console.log("`initialize` tx signature:", tx);

    const globalConfigAccount = await getGlobalConfigAccount(program);

    console.log("Global config account:", globalConfigAccount);
  });

  it("`create_escrow`!", async () => {
    const amount = bn(10_000_000); // 10
    const tx = await program.methods
      .createEscrow(amount)
      .accounts({
        mint: randomMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("`create_escrow` tx signature:", tx);

    const globalConfigAccount = await getGlobalConfigAccount(program);

    console.log("Global config account after creating escrow:", globalConfigAccount);

    const escrowAccount = await getEscrowAccount(
      program,
      globalConfigAccount.escrowCount.toNumber() - 1
    );

    console.log("Escrow account:", escrowAccount);
    console.log(escrowAccount.amount.toNumber());
  });
});
