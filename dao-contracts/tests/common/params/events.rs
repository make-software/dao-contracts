use cucumber::Parameter;

use super::{account::Account, common::U256, voting::Choice, Contract};
use crate::common::helpers;

#[derive(Debug, Parameter)]
#[param(name = "event", regex = ".+")]
pub enum Event {
    OwnerChanged(Account),
    AddedToWhitelist(Account),
    RemovedFromWhitelist(Account),
    VotingContractCreated(Contract, Contract, Contract),
    VotingCreated(Account, u32, u32, Option<u32>, u32, u64, u32, u64),
    BallotCast(Account, u32, Choice, U256),
    NftTransfer(Option<Account>, Option<Account>, U256),
    NftApproval(Option<Account>, Option<Account>, U256),
}

impl From<&Vec<String>> for Event {
    fn from(value: &Vec<String>) -> Self {
        let event_name = value[0].as_str();
        match event_name {
            "OwnerChanged" => {
                let account: Account = value.get(1).into();
                Self::OwnerChanged(account)
            }
            "AddedToWhitelist" => {
                let account: Account = value.get(1).into();
                Self::AddedToWhitelist(account)
            }
            "RemovedFromWhitelist" => {
                let account: Account = value.get(1).into();
                Self::RemovedFromWhitelist(account)
            }
            "Approval" => {
                let from = helpers::parse_or_none(value.get(1));
                let to = helpers::parse_or_none(value.get(2));
                let token_id = helpers::parse(value.get(3), "Couldn't parse token id");
                Self::NftApproval(from, to, token_id)
            }
            "Transfer" => {
                let from = helpers::parse_or_none(value.get(1));
                let to = helpers::parse_or_none(value.get(2));
                let token_id = helpers::parse(value.get(3), "Couldn't parse token id");
                Self::NftTransfer(from, to, token_id)
            }
            "VotingContractCreated" => {
                let variable_repo = helpers::parse(value.get(1), "Couldn't parse contract");
                let reputation_token = helpers::parse(value.get(2), "Couldn't parse contract");
                let kyc_voter = helpers::parse(value.get(3), "Couldn't parse contract");
                Self::VotingContractCreated(variable_repo, reputation_token, kyc_voter)
            }
            "VotingCreated" => {
                let voter = helpers::parse(value.get(1), "Couldn't parse contract");
                let voting_id = helpers::parse_or_default(value.get(2));
                let informal_voting_id = helpers::parse_or_default(value.get(3));
                let formal_voting_id = helpers::parse_or_none(value.get(4));
                let formal_voting_quorum = helpers::parse_or_default(value.get(5));
                let formal_voting_time = helpers::parse_or_default(value.get(6));
                let informal_voting_quorum = helpers::parse_or_default(value.get(7));
                let informal_voting_time = helpers::parse_or_default(value.get(8));
                Self::VotingCreated(
                    voter,
                    voting_id,
                    informal_voting_id,
                    formal_voting_id,
                    formal_voting_quorum,
                    formal_voting_time,
                    informal_voting_quorum,
                    informal_voting_time,
                )
            }
            "BallotCast" => {
                let voter = helpers::parse(value.get(1), "Couldn't parse contract");
                let voting_id = helpers::parse_or_default(value.get(2));
                let choice = helpers::parse(value.get(3), "Couldn't parse choice");
                let stake = helpers::parse_or_default(value.get(4));
                Self::BallotCast(voter, voting_id, choice, stake)
            }
            invalid => panic!("Unknown event {}", invalid),
        }
    }
}
