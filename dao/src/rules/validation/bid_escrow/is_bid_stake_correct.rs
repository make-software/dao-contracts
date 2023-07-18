use crate::rules::validation::Validation;
use crate::utils::Error;
use macros::Rule;
use odra::contract_env::attached_value;
use odra::types::Balance;

/// Verifies if the worker's stake is correct.
/// May return [Error::NotOnboardedWorkerMustStakeCSPR], [Error::ZeroStake],
/// [Error::OnboardedWorkerCannotStakeCSPR], [Error::CannotStakeBothCSPRAndReputation],
/// or [Error::AttachedValueMismatch].
#[derive(Rule)]
pub struct IsBidStakeCorrect {
    is_worker_va: bool,
    cspr_stake: Option<Balance>,
    reputation_stake: Balance,
}

impl Validation for IsBidStakeCorrect {
    fn validate(&self) -> Result<(), Error> {
        match self.cspr_stake {
            None => {
                // Worker doesn't stake cspr, so it must be a VA and must stake reputation.
                if !self.is_worker_va {
                    return Err(Error::NotOnboardedWorkerMustStakeCSPR);
                }
                if self.reputation_stake == Balance::zero() {
                    return Err(Error::ZeroStake);
                }
            }
            Some(cspr_stake) => {
                // Worker staked cspr, so it must not be a VA and must not stake reputation.
                if self.is_worker_va {
                    return Err(Error::OnboardedWorkerCannotStakeCSPR);
                }
                if self.reputation_stake > Balance::zero() {
                    return Err(Error::CannotStakeBothCSPRAndReputation);
                }
                if cspr_stake == Balance::zero() {
                    return Err(Error::ZeroStake);
                }
                if attached_value() != cspr_stake {
                    return Err(Error::AttachedValueMismatch);
                }
            }
        }

        Ok(())
    }
}
