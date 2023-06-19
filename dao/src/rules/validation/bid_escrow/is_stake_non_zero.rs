use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::types::Balance;

/// Makes sure the stake is non-zero. May return [Error::ZeroStake].
#[derive(Rule)]
pub struct IsStakeNonZero {
    reputation_stake: Balance,
    cspr_stake: Option<Balance>,
}

impl Validation for IsStakeNonZero {
    fn validate(&self) -> Result<(), Error> {
        if self.cspr_stake.is_none() && self.reputation_stake.is_zero() {
            return Err(Error::ZeroStake);
        }
        Ok(())
    }
}
