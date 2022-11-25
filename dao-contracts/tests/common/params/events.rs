use cucumber::Parameter;

use super::{account::Account, common::U256};
use crate::common::helpers;

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
                let from = helpers::parse_or_none::<Account>(value.get(1));
                let to = helpers::parse_or_none::<Account>(value.get(2));
                let token_id = helpers::parse::<U256>(value.get(3), "Couldn't parse token id");
                Self::NtfEvent(NtfEvent::Approval(from, to, token_id))
            }
            "Transfer" => {
                let from = helpers::parse_or_none::<Account>(value.get(1));
                let to = helpers::parse_or_none::<Account>(value.get(2));
                let token_id = helpers::parse::<U256>(value.get(3), "Couldn't parse token id");
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
