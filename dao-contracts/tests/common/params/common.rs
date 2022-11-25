use std::str::FromStr;

use cucumber::Parameter;

use super::nft::Account;
use crate::common::helpers;

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter, PartialEq, Eq)]
#[param(name = "u256", regex = r"\d+")]
pub struct U256(pub casper_types::U256);

impl U256 {
    pub fn zero() -> Self {
        U256(casper_types::U256::zero())
    }

    pub fn one() -> Self {
        U256(casper_types::U256::one())
    }
}

#[derive(Debug, Parameter)]
#[param(name = "contract", regex = ".+")]
pub enum Contract {
    KycToken,
    VaToken,
    ReputationToken,
    BidEscrow,
    VariableRepository,
    SlashingVoter,
}

impl FromStr for Contract {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contract = match s {
            "KycToken" => Self::KycToken,
            "VaToken" => Self::VaToken,
            "ReputationToken" => Self::ReputationToken,
            "BidEscrow" => Self::BidEscrow,
            "VariableRepository" => Self::VariableRepository,
            "SlashingVoter" => Self::SlashingVoter,
            invalid => return Err(format!("Unknown contract {}", invalid)),
        };
        Ok(contract)
    }
}

#[derive(Debug, Parameter)]
#[param(name = "event", regex = ".+")]
pub enum Event {
    OwnerChanged(Account),
    AddedToWhitelist(Account),
    RemovedFromWhitelist(Account),
    NtfEvent(NtfEvent),
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
                let from = helpers::parse_option::<Account>(value.get(1));
                let to = helpers::parse_option::<Account>(value.get(2));
                let token_id = helpers::parse::<U256>(value.get(3), "");
                Self::NtfEvent(NtfEvent::Approval(from, to, token_id))
            }
            "Transfer" => {
                let from = helpers::parse_option::<Account>(value.get(1));
                let to = helpers::parse_option::<Account>(value.get(2));
                let token_id = helpers::parse::<U256>(value.get(3), "");
                Self::NtfEvent(NtfEvent::Transfer(from, to, token_id))
            }
            invalid => panic!("Unknown event {}", invalid),
        }
    }
}

#[derive(Debug)]
pub enum NtfEvent {
    Transfer(Option<Account>, Option<Account>, U256),
    Approval(Option<Account>, Option<Account>, U256),
}
