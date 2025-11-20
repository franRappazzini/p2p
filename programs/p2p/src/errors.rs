use anchor_lang::error_code;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::example_mocks::solana_signature::Signature;

#[error_code]
pub enum P2pError {
    #[msg("The escrow has already been taken.")]
    EscrowAlreadyTaken,
    #[msg("Signature verification failed.")]
    SignatureVerificationFailed,
}
