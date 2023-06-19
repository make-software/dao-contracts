use cucumber::Parameter;
use odra::types::Balance as OdraBalance;
use std::{ops::Deref, str::FromStr};

// Macro that is used to implement CsprBalance and ReputationBalance.
macro_rules! impl_balance {
    ($type:ident, $name:expr) => {
        #[derive(
            Copy,
            Clone,
            Debug,
            Default,
            derive_more::Deref,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Parameter,
        )]
        #[param(name = $name, regex = r"\d+")]
        pub struct $type(pub OdraBalance);

        impl FromStr for $type {
            type Err = String;

            fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
                let value = OdraBalance::from((s.parse::<f32>().unwrap() * 1_000f32) as u32)
                    * OdraBalance::from(1_000_000);
                Ok(Self(value))
            }
        }

        impl From<u32> for $type {
            fn from(value: u32) -> Self {
                Self(OdraBalance::from(value))
            }
        }

        impl From<OdraBalance> for $type {
            fn from(value: OdraBalance) -> Self {
                Self(value)
            }
        }

        #[allow(dead_code)]
        impl $type {
            pub fn zero() -> Self {
                Self(OdraBalance::zero())
            }

            pub fn one() -> Self {
                Self(OdraBalance::from(1_000_000_000))
            }
        }
    };
}

impl_balance!(CsprBalance, "balance");
impl_balance!(ReputationBalance, "reputation");

#[derive(Clone, Copy, Debug, Default, derive_more::Deref, Parameter, PartialEq)]
#[param(name = "token_id", regex = r"\d+")]
pub struct TokenId(pub dao::core_contracts::TokenId);

impl FromStr for TokenId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let num: u32 = s.parse().map_err(|_| format!("Invalid token id: {}", s))?;
        Ok(Self(dao::core_contracts::TokenId::from(num)))
    }
}

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
            invalid => {
                panic!("Unknown unit {:?} option - it should be either seconds, minutes, hours or days", invalid)
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
