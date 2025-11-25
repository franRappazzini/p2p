use anchor_lang::prelude::*;

use crate::{constants::DISCRIMINATOR_SIZE, errors::P2pError};

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub state: EscrowState,
    pub disputed_by: EscrowDisputedBy,
    pub bump: u8,
}

impl Escrow {
    pub const SIZE: usize = DISCRIMINATOR_SIZE + Escrow::INIT_SPACE;

    pub fn can_cancel(&self, fiat_deadline_secs: i64) -> bool {
        if let EscrowState::Open(timestamp) = self.state {
            Clock::get().unwrap().unix_timestamp > timestamp + fiat_deadline_secs
        } else {
            false
        }
    }

    pub fn dispute(&mut self, dispute_deadline_secs: i64, disputant: Pubkey) -> Result<()> {
        let current_timestamp = Clock::get()?.unix_timestamp;

        match self.state {
            EscrowState::FiatPaid(timestamp) => {
                if current_timestamp < timestamp + dispute_deadline_secs {
                    return Err(P2pError::CannotDisputeEscrow.into());
                }

                if disputant == self.seller {
                    self.disputed_by = EscrowDisputedBy::Seller;
                } else if disputant == self.buyer {
                    self.disputed_by = EscrowDisputedBy::Buyer;
                } else {
                    return Err(P2pError::UnauthorizedDispute.into());
                }

                self.state = EscrowState::Dispute(current_timestamp);
                Ok(())
            }
            EscrowState::Dispute(timestamp) => {
                if current_timestamp < timestamp + dispute_deadline_secs {
                    return Err(P2pError::CannotDisputeEscrow.into());
                }

                // Only the counterpart can redispute
                if disputant == self.seller && self.disputed_by == EscrowDisputedBy::Buyer {
                    self.disputed_by = EscrowDisputedBy::Seller;
                    self.state = EscrowState::ReDispute(current_timestamp);
                    Ok(())
                } else if disputant == self.buyer && self.disputed_by == EscrowDisputedBy::Seller {
                    self.disputed_by = EscrowDisputedBy::Buyer;
                    self.state = EscrowState::ReDispute(current_timestamp);
                    Ok(())
                } else {
                    Err(P2pError::EscrowAlreadyInDispute.into())
                }
            }
            EscrowState::ReDispute(_) => Err(P2pError::EscrowAlreadyInDispute.into()),
            EscrowState::Open(_) => Err(P2pError::EscrowIsNotTaken.into()),
        }
    }
}

// data = timestamp
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum EscrowState {
    Open(i64),
    FiatPaid(i64),
    Dispute(i64),
    ReDispute(i64),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum EscrowDisputedBy {
    Nobody,
    Seller,
    Buyer,
}
