use casper_dao_utils::Error;

use crate::voting::voting_state_machine::VotingStateMachine;

pub trait Validation {
    fn validate(&self) -> Result<(), Error>;
}

pub trait VotingValidation {
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error>;
}
