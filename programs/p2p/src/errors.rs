use anchor_lang::error_code;

#[error_code]
pub enum P2pError {
    #[msg("The escrow has already been taken.")]
    EscrowAlreadyTaken,
    #[msg("Signature verification failed.")]
    SignatureVerificationFailed,
    #[msg("No available funds to withdraw.")]
    NoAvailableFundsToWithdraw,
    #[msg("The escrow cannot be canceled yet. Please wait until the fiat deadline has passed.")]
    CannotCancelEscrowYet,
    #[msg("Invalid escrow state for this operation.")]
    InvalidEscrowState,
}
