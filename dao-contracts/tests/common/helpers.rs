use std::{fmt::Debug, str::FromStr};

use casper_dao_utils::{consts::*, BlockTime};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U512,
};

use super::params::{Balance, TimeUnit};

pub fn parse_bool(value: String) -> bool {
    match value.as_str() {
        "with" => true,
        "without" => false,
        "is" => true,
        "isn't" => false,
        "yes" => true,
        "no" => false,
        _ => {
            panic!("Unknown with option");
        }
    }
}

/// Converts a string value from Gherkin scenario to a `Bytes` representation of the value
pub fn value_to_bytes(value: &str, key: &str) -> Bytes {
    match value {
        "true" | "false" => value.parse::<bool>().unwrap().to_bytes().unwrap().into(),
        _ => match key {
            POST_JOB_DOS_FEE
            | BID_ESCROW_PAYMENT_RATIO
            | DEFAULT_POLICING_RATE
            | REPUTATION_CONVERSION_RATE
            | BID_ESCROW_INFORMAL_QUORUM_RATIO
            | BID_ESCROW_FORMAL_QUORUM_RATIO
            | INFORMAL_QUORUM_RATIO
            | FORMAL_QUORUM_RATIO
            | DEFAULT_REPUTATION_SLASH
            | VOTING_CLEARNESS_DELTA => {
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

pub fn to_milliseconds(value: BlockTime, unit: TimeUnit) -> BlockTime {
    let seconds = match unit {
        TimeUnit::Seconds => value,
        TimeUnit::Minutes => value * 60,
        TimeUnit::Hours => value * 60 * 60,
        TimeUnit::Days => value * 60 * 60 * 24,
    };
    seconds * 1000u64
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
