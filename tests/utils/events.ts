import * as anchor from "@coral-xyz/anchor";

import { P2p } from "../../target/types/p2p";

function createEventListeners(program: anchor.Program<P2p>) {
  const createEscrowListener = program.addEventListener("escrowCreated", (event) => {
    console.log("Escrow Created Event:", event.id);
  });

  const takeEscrowListener = program.addEventListener("escrowTaken", (event) => {
    console.log("Escrow Taken Event:", event.id);
  });

  const tokensReleasedListener = program.addEventListener("tokensReleased", (event) => {
    console.log("Tokens Released Event:", event.id);
  });

  return [createEscrowListener, takeEscrowListener, tokensReleasedListener];
}

async function removeEventListener(program: anchor.Program<P2p>, listeners: number[]) {
  for (const listener of listeners) {
    await program.removeEventListener(listener);
  }
}

export { createEventListeners, removeEventListener };
