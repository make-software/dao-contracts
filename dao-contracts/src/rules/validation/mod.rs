mod is_user_kyced;

use casper_dao_utils::Error;
pub use is_user_kyced::IsUserKyced;

use crate::voting::voting_state_machine::VotingStateMachine;

pub trait Validation {
    fn validate(&self) -> Result<(), Error>;
}

pub trait VotingValidation {
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error>;
}
