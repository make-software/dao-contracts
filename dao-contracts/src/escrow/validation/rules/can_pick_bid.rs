use casper_dao_utils::{Address, Error};

use crate::rules::validation::Validation;

pub struct CanPickBid {
    pub caller: Address,
    pub job_poster: Address,
}

impl Validation for CanPickBid {
    fn validate(&self) -> Result<(), Error> {
        if self.job_poster != self.caller {
            return Err(Error::OnlyJobPosterCanPickABid);
        }
        Ok(())
    }
}

impl CanPickBid {
    pub fn create(caller: Address, job_poster: Address) -> Box<CanPickBid> {
        Box::new(Self { caller, job_poster })
    }
}
