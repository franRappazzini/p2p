import * as anchor from "@coral-xyz/anchor";

import {
  DISPUTE_DEADLINE_SECS,
  DISPUTE_FEE_ESCROW,
  FEE_BPS,
  FIAT_DEADLINE_SECS,
} from "./utils/constants";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { createEventListeners, removeEventListener } from "./utils/events";
import {
  getAllEscrowAccounts,
  getDisputeVaultAccount,
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

  const eventListeners = []; // createEventListeners(program);

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
    const tx = await program.methods
      .initialize(FEE_BPS, FIAT_DEADLINE_SECS, DISPUTE_DEADLINE_SECS, DISPUTE_FEE_ESCROW)
      .rpc();
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

    const mintVaultAccount = await getMintVaultAccount(program, randomMint);
    expect(mintVaultAccount.availableAmount).to.greaterThan(0);

    try {
      await getEscrowAccount(program, id);
      expect.fail("Escrow account should be closed after token release");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }
  });

  it("`cancel_escrow`!", async () => {
    // First, create a new escrow
    const amount = bn(15_000_000); // 15
    const createTx = await program.methods
      .createEscrow(amount)
      .accounts({
        buyer: randomBuyer.publicKey,
        mint: randomMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // create a await to simulate time passing before cancelling
    await new Promise((resolve) => setTimeout(resolve, 3000));

    const id = 1; // second escrow
    const tx = await program.methods
      .cancelEscrow(bn(id))
      .accounts({ tokenProgram: TOKEN_PROGRAM_ID })
      .rpc();

    console.log("`cancel_escrow` tx signature:", tx);

    try {
      await getEscrowAccount(program, id);
      expect.fail("Escrow account should be closed after cancellation");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }
  });

  it("`create_dispute` (and re-dispute)!", async () => {
    // First, create a new escrow
    const amount = bn(20_000_000); // 20
    const createTx = await program.methods
      .createEscrow(amount)
      .accounts({
        buyer: randomBuyer.publicKey,
        mint: randomMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const id = 2; // third escrow

    const markEscrowAsPaidTx = await program.methods
      .markEscrowAsPaid(bn(id))
      .accounts({ buyer: randomBuyer.publicKey })
      .signers([randomBuyer])
      .rpc();

    // create a await to simulate time passing before creating dispute
    await new Promise((resolve) => setTimeout(resolve, 3000));

    // Now, create a dispute on that escrow
    const tx = await program.methods
      .createDispute(bn(id))
      .accounts({ disputant: randomBuyer.publicKey })
      .signers([randomBuyer])
      .rpc();

    console.log("`create_dispute` tx signature:", tx);

    const escrowAccount = await getEscrowAccount(program, id);
    const disputeVaultAccount = await getDisputeVaultAccount(connection, program);

    expect(escrowAccount.state).to.equal("dispute");
    expect(disputeVaultAccount?.lamports).to.greaterThan(DISPUTE_FEE_ESCROW.toNumber());

    // create a await to simulate time passing before creating dispute
    await new Promise((resolve) => setTimeout(resolve, 3000));

    const reTx = await program.methods.createDispute(bn(id)).rpc();

    console.log("`re-create_dispute` tx signature:", reTx);

    const escrowAccountAfterRe = await getEscrowAccount(program, id);
    const disputeVaultAccountAfterRe = await getDisputeVaultAccount(connection, program);

    expect(escrowAccountAfterRe.state).to.equal("reDispute");
    expect(disputeVaultAccountAfterRe?.lamports).to.greaterThan(2 * DISPUTE_FEE_ESCROW.toNumber());
  });

  it("`resolve_dispute`!", async () => {
    await new Promise((resolve) => setTimeout(resolve, 3000));
    const mintVaultAccountBefore = await getMintVaultAccount(program, randomMint);

    const id = 2; // third escrow
    const to = wallet.publicKey;
    const toSeller = to == wallet.publicKey;

    const tx = await program.methods
      .resolveDispute(bn(id))
      .accounts({ to, tokenProgram: TOKEN_PROGRAM_ID })
      .rpc();

    console.log("`resolve_dispute` tx signature:", tx);

    const globalConfigAccount = await getGlobalConfigAccount(program);
    console.log({ globalConfigAccount });
    expect(globalConfigAccount.availableLamports).to.equal(DISPUTE_FEE_ESCROW.toNumber());

    const disputeVaultAccount = await getDisputeVaultAccount(connection, program);
    console.log({ disputeVaultAccount });
    expect(disputeVaultAccount?.lamports)
      .to.lessThan(DISPUTE_FEE_ESCROW.toNumber() * 2)
      .to.greaterThan(DISPUTE_FEE_ESCROW.toNumber());

    const mintVaultAccount = await getMintVaultAccount(program, randomMint);
    console.log({ mintVaultAccount });
    expect(mintVaultAccount.availableAmount).to.greaterThanOrEqual(
      mintVaultAccountBefore.availableAmount
    );

    try {
      await getEscrowAccount(program, id);
      expect.fail("Escrow account should be closed after dispute resolution");
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
    expect(mintVaultAccount.availableAmount).to.equal(0);
  });

  after(async () => {
    await removeEventListener(program, eventListeners);
  });
});
