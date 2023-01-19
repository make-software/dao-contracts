use casper_dao_utils::{casper_dao_macros::Rule, Error};
use casper_types::U512;

use crate::rules::validation::Validation;

#[derive(Rule)]
pub struct IsStakeNonZero {
    reputation_stake: U512,
    cspr_stake: Option<U512>,
}

impl Validation for IsStakeNonZero {
    fn validate(&self) -> Result<(), Error> {
        if self.cspr_stake.is_none() && self.reputation_stake.is_zero() {
            return Err(Error::ZeroStake);
        }
        Ok(())
    }
}
