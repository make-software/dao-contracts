//! Groups validations.
mod is_user_kyced;

pub mod bid_escrow;
pub mod voting;

use casper_dao_utils::Error;
pub use is_user_kyced::IsUserKyced;

use crate::voting::voting_state_machine::VotingStateMachine;

/// A generic validation.
pub trait Validation {
    /// Returns the result of validation.
    fn validate(&self) -> Result<(), Error>;
}

/// A validation in the voting context.
pub trait VotingValidation {
    /// Returns the result of validation.
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error>;
}
