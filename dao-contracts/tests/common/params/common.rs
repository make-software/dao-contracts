use std::{ops::Deref, str::FromStr};

use cucumber::Parameter;

use crate::common::helpers::{to_cspr, to_rep};

#[derive(
    Copy, Clone, Debug, Default, derive_more::Deref, Parameter, PartialEq, Eq, PartialOrd, Ord,
)]
#[param(name = "u256", regex = r"\d+")]
pub struct U256(pub casper_types::U256);

#[allow(dead_code)]
impl U256 {
    pub fn zero() -> Self {
        U256(casper_types::U256::zero())
    }

    pub fn one() -> Self {
        U256(casper_types::U256::one())
    }
}

impl FromStr for U256 {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        casper_types::U256::from_dec_str(s)
            .map_err(|_| "Err".to_string())
            .map(U256)
    }
}
#[derive(
    Copy, Clone, Debug, Default, derive_more::Deref, Parameter, PartialEq, Eq, PartialOrd, Ord,
)]
#[param(name = "u512", regex = r"\d+")]
pub struct U512(pub casper_types::U512);

#[allow(dead_code)]
impl U512 {
    pub fn zero() -> Self {
        U512(casper_types::U512::zero())
    }

    pub fn one() -> Self {
        U512(casper_types::U512::one())
    }
}

impl FromStr for U512 {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(U512(to_cspr(s)))
    }
}

#[derive(
    Copy, Clone, Debug, Default, derive_more::Deref, PartialEq, Eq, PartialOrd, Ord, Parameter,
)]
#[param(name = "balance", regex = r"\d+")]
pub struct Balance(pub casper_types::U256);

impl FromStr for Balance {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(Balance(to_rep(s)))
    }
}

impl From<U256> for Balance {
    fn from(value: U256) -> Self {
        Balance(value.0 * 1_000_000_000)
    }
}

#[derive(
    Copy, Clone, Debug, Default, derive_more::Deref, PartialEq, Eq, PartialOrd, Ord, Parameter,
)]
#[param(name = "cspr", regex = r"\d+")]
pub struct CsprBalance(pub casper_types::U512);

impl FromStr for CsprBalance {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(CsprBalance(to_cspr(s)))
    }
}

impl From<U512> for CsprBalance {
    fn from(value: U512) -> Self {
        CsprBalance(value.0 * 1_000_000_000)
    }
}

#[derive(Debug, Default, derive_more::FromStr, derive_more::Deref, Parameter)]
#[param(name = "token_id", regex = r"\d+")]
pub struct TokenId(pub casper_dao_erc721::TokenId);

#[derive(Debug, Parameter)]
#[param(name = "time_unit", regex = r".*")]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl FromStr for TimeUnit {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(match s {
            "seconds" | "second" => Self::Seconds,
            "minutes" | "minute" => Self::Minutes,
            "hours" | "hour" => Self::Hours,
            "days" | "day" => Self::Days,
            _ => {
                panic!("Unknown unit option - it should be either seconds, minutes, hours or days")
            }
        })
    }
}

#[derive(Debug, Parameter)]
#[param(name = "result", regex = r"succeeds|fails")]
pub enum Result {
    Success,
    Failure,
}

impl FromStr for Result {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Ok(match s {
            "succeeds" => Self::Success,
            "fails" => Self::Failure,
            _ => panic!("Unknown result option - it should be either succeeds or fails"),
        })
    }
}

impl Deref for Result {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            Result::Success => &true,
            Result::Failure => &false,
        }
    }
}
