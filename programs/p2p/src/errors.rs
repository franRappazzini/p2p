use anchor_lang::error_code;

#[error_code]
pub enum P2pError {
    #[msg("The escrow has already been taken.")]
    EscrowAlreadyTaken,
    #[msg("Signature verification failed.")]
    SignatureVerificationFailed,
    #[msg("No available funds to withdraw.")]
    NoAvailableFundsToWithdraw,
}
