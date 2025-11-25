use anchor_lang::error_code;

#[error_code]
pub enum P2pError {
    #[msg("The escrow has already been taken.")]
    EscrowAlreadyTaken,
    #[msg("Signature verification failed.")]
    SignatureVerificationFailed,
    #[msg("No available funds to withdraw.")]
    NoAvailableFundsToWithdraw,
    #[msg("The escrow cannot be canceled.")]
    CannotCancelEscrow,
    #[msg("Invalid escrow state for this operation.")]
    InvalidEscrowState,
    #[msg("The escrow cannot be disputed.")]
    CannotDisputeEscrow,
    #[msg("The escrow is not taken yet.")]
    EscrowIsNotTaken,
    #[msg("Unauthorized disputant.")]
    UnauthorizedDispute,
    #[msg("The escrow is already in dispute.")]
    EscrowAlreadyInDispute,
}
