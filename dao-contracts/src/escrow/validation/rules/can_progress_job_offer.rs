use casper_dao_utils::{Address, Error};

use crate::rules::validation::Validation;

pub struct CanProgressJobOffer {
    pub caller: Address,
    pub job_poster: Address,
}

impl Validation for CanProgressJobOffer {
    fn validate(&self) -> Result<(), Error> {
        if self.job_poster != self.caller {
            return Err(Error::OnlyJobPosterCanPickABid);
        }
        Ok(())
    }
}

impl CanProgressJobOffer {
    pub fn create(caller: Address, job_poster: Address) -> Box<CanProgressJobOffer> {
        Box::new(Self { caller, job_poster })
    }
}
