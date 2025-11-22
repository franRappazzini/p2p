import * as anchor from "@coral-xyz/anchor";

interface GlobalConfigParserParams {
  authority: anchor.web3.PublicKey;
  escrowCount: anchor.BN;
  feeBps: number;
  fiatDeadlineSecs: anchor.BN;
  bump: number;
}

function globalConfigParser(params: GlobalConfigParserParams) {
  return {
    authority: params.authority.toString(),
    escrowCount: params.escrowCount.toNumber(),
    feeBps: params.feeBps,
    fiatDeadlineSecs: params.fiatDeadlineSecs.toNumber(),
    bump: params.bump,
  };
}

interface EscrowParserParams {
  id: anchor.BN;
  seller: anchor.web3.PublicKey;
  buyer: anchor.web3.PublicKey;
  mint: anchor.web3.PublicKey;
  amount: anchor.BN;
  state: { [kind: string]: { "0": anchor.BN } };
  bump: number;
}

function escrowParser(params: EscrowParserParams) {
  return {
    id: params.id.toNumber(),
    seller: params.seller.toString(),
    buyer: params.buyer.toString(),
    mint: params.mint.toString(),
    amount: params.amount.toNumber(),
    state: Object.keys(params.state)[0],
    timestamp: Object.values(Object.values(params.state)[0])[0].toNumber(),
    bump: params.bump,
  };
}

export { globalConfigParser, escrowParser };
