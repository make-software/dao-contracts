use casper_dao_utils::BlockTime;

use crate::{
    rules::{builder::RulesBuilder, Rules},
    voting::{
        validation::rules::{
            after_formal_voting::AfterFormalVoting,
            vote_in_time::VoteInTime,
            voting_not_completed::VotingNotCompleted,
        },
        voting_state_machine::VotingStateMachine,
    },
};

pub mod rules;
mod voting_state_machine;
