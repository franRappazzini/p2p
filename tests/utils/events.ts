import * as anchor from "@coral-xyz/anchor";

import { P2p } from "../../target/types/p2p";

function createEventListeners(program: anchor.Program<P2p>) {
  const createEscrowListener = program.addEventListener("escrowCreated", (event) => {
    console.log("Escrow Created Event:", event.id);
  });

  const markEscrowAsPaidListener = program.addEventListener("markEscrowAsPaid", (event) => {
    console.log("Escrow Marked As Paid Event:", event.id);
  });

  const tokensReleasedListener = program.addEventListener("tokensReleased", (event) => {
    console.log("Tokens Released Event:", event.id);
  });

  const cancelEscrowListener = program.addEventListener("escrowCancelled", (event) => {
    console.log("Escrow Cancelled Event:", event.id);
  });

  // DisputeCreated and DisputeResolved

  return [
    createEscrowListener,
    markEscrowAsPaidListener,
    tokensReleasedListener,
    cancelEscrowListener,
  ];
}

async function removeEventListener(program: anchor.Program<P2p>, listeners: number[]) {
  for (const listener of listeners) {
    await program.removeEventListener(listener);
  }
}

export { createEventListeners, removeEventListener };
