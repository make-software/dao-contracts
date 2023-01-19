use casper_dao_utils::{casper_dao_macros::Rule, BlockTime, Error};

use crate::{bid_escrow::job::JobStatus, rules::validation::Validation};

#[derive(Rule)]
pub struct IsGracePeriod {
    job_status: JobStatus,
    job_finish_time: BlockTime,
    block_time: BlockTime,
}

impl Validation for IsGracePeriod {
    fn validate(&self) -> Result<(), Error> {
        if self.job_status != JobStatus::Created {
            return Err(Error::CannotSubmitJobProof);
        }

        if self.job_finish_time >= self.block_time {
            return Err(Error::GracePeriodNotStarted);
        }
        Ok(())
    }
}
