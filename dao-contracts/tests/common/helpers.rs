use std::{fmt::Debug, str::FromStr};

use casper_dao_utils::BlockTime;
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U512,
};

use super::params::{Balance, TimeUnit};

/// Converts a string value from Gherkin scenario to a `Bytes` representation of the value
pub fn value_to_bytes(value: &str, key: &str) -> Bytes {
    match value {
        "true" => true.to_bytes().unwrap().into(),
        "false" => false.to_bytes().unwrap().into(),
        _ => match key {
            "PostJobDosFee" | "GovernancePaymentRatio" => {
                let value = U512::from_dec_str(value).unwrap();
                Bytes::from(value.to_bytes().unwrap())
            }
            "DefaultPolicingRate"
            | "ReputationConversionRate"
            | "GovernanceInformalQuorumRatio"
            | "GovernanceFormalQuorumRatio"
            | "InformalQuorumRatio"
            | "FormalQuorumRatio"
            | "DefaultReputationSlash"
            | "VotingClearnessDelta" => {
                let value = U512::from_dec_str(value).unwrap();
                Bytes::from(value.to_bytes().unwrap())
            }
            _ => {
                let value: u64 = value.parse().unwrap();
                Bytes::from(value.to_bytes().unwrap())
            }
        },
    }
}

pub fn is_balance_close_enough<A: Into<Balance>, B: Into<Balance>>(a: A, b: B) -> bool {
    let a: Balance = a.into();
    let b: Balance = b.into();
    let (a, b) = (a.0, b.0);
    let diff = if a > b { a - b } else { b - a };
    diff < U512::from(10_000_000)
}

pub fn to_seconds(value: BlockTime, unit: TimeUnit) -> BlockTime {
    match unit {
        TimeUnit::Seconds => value,
        TimeUnit::Minutes => value * 60,
        TimeUnit::Hours => value * 60 * 60,
        TimeUnit::Days => value * 60 * 60 * 24,
    }
}

pub fn parse<T>(item: Option<&String>, err_msg: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    item.expect(err_msg).parse::<T>().expect("Parsing failed.")
}

pub fn parse_or_default<T: FromStr + Default>(item: Option<&String>) -> T {
    match item {
        Some(value) => value.parse::<T>().unwrap_or_default(),
        None => T::default(),
    }
}

pub fn parse_or_none<T: FromStr>(item: Option<&String>) -> Option<T> {
    match item {
        Some(value) => {
            if value.is_empty() {
                None
            } else {
                value.parse::<T>().ok()
            }
        }
        None => None,
    }
}
