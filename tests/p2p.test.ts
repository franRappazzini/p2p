import * as anchor from "@coral-xyz/anchor";

import { FEE_BPS, FIAT_DEADLINE_SECS } from "./utils/constants";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { createEventListeners, removeEventListener } from "./utils/events";
import {
  getAllEscrowAccounts,
  getEscrowAccount,
  getGlobalConfigAccount,
  getMintVaultAccount,
} from "./utils/accounts";

import { P2p } from "../target/types/p2p";
import { Program } from "@coral-xyz/anchor";
import { bn } from "./utils/functions";
import { decodeUTF8 } from "tweetnacl-util";
import { expect } from "chai";
import nacl from "tweetnacl";

describe("p2p", () => {
  const provider = anchor.AnchorProvider.env();
  const { connection, wallet } = provider;

  anchor.setProvider(provider);

  const program = anchor.workspace.p2p as Program<P2p>;

  let randomMint: anchor.web3.PublicKey;
  const randomBuyer = anchor.web3.Keypair.generate();

  const eventListeners = createEventListeners(program);

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

    await connection.requestAirdrop(randomBuyer.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
  });

  it("`initialize`!", async () => {
    const tx = await program.methods.initialize(FEE_BPS, FIAT_DEADLINE_SECS).rpc();
    console.log("`initialize` tx signature:", tx);

    const globalConfigAccount = await getGlobalConfigAccount(program);

    console.log("Global config account:", globalConfigAccount);
  });

  it("`create_escrow`!", async () => {
    const amount = bn(10_000_000); // 10
    const tx = await program.methods
      .createEscrow(amount)
      .accounts({
        buyer: randomBuyer.publicKey,
        mint: randomMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("`create_escrow` tx signature:", tx);

    const globalConfigAccount = await getGlobalConfigAccount(program);

    const escrowAccount = await getEscrowAccount(program, globalConfigAccount.escrowCount - 1);
    console.log({ escrowAccount });

    expect(globalConfigAccount.escrowCount).to.equal(1);
    expect(escrowAccount.amount).to.equal(amount.toNumber());
    expect(escrowAccount.mint).to.equal(randomMint.toString());
    expect(escrowAccount.seller).to.equal(wallet.publicKey.toString());
    expect(escrowAccount.buyer).to.equal(randomBuyer.publicKey.toString());
  });

  it("`mark_escrow_as_paid`!", async () => {
    const id = bn(0);
    const tx = await program.methods
      .markEscrowAsPaid(id)
      .accounts({ buyer: randomBuyer.publicKey })
      .signers([randomBuyer])
      .rpc();

    console.log("`mark_escrow_as_paid` tx signature:", tx);

    const escrowAccount = await getEscrowAccount(program, id.toNumber());

    expect(escrowAccount.seller).to.equal(wallet.publicKey.toString());
    expect(escrowAccount.buyer).to.equal(randomBuyer.publicKey.toString());
    expect(escrowAccount.state).to.equal("fiatPaid");
  });

  it("`release_tokens_in_escrow`!", async () => {
    const id = 0;
    const escrows = await getAllEscrowAccounts(program);

    // Create the message and sign it with the buyer wallet
    const message = `approve_release:${escrows[id].publicKey.toString()}`;
    const messageBytes = decodeUTF8(message);
    const signature = nacl.sign.detached(messageBytes, wallet.payer?.secretKey);
    const isValid = nacl.sign.detached.verify(messageBytes, signature, wallet.publicKey.toBytes());
    expect(isValid).to.be.true;

    const tx = await program.methods
      .releaseTokensInEscrow(bn(id), Array.from(signature))
      .accounts({
        buyer: randomBuyer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([randomBuyer])
      .rpc();

    console.log("`release_tokens_in_escrow` tx signature:", tx);

    try {
      await getEscrowAccount(program, id);
      expect.fail("Escrow account should be closed after token release");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }
  });

  it("`withdraw_spl`!", async () => {
    const tx = await program.methods
      .withdrawSpl()
      .accounts({
        mint: randomMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("`withdraw_spl` tx signature:", tx);

    const mintVaultAccount = await getMintVaultAccount(program, randomMint);
    expect(mintVaultAccount.availableAmount.toNumber()).to.equal(0);
  });

  after(async () => {
    await removeEventListener(program, eventListeners);
  });
});
