

use casper_dao_utils::{Address, Error};


use crate::{rules::validation::Validation};

pub struct CanBidOnOwnJob {
    pub worker: Address,
    pub job_poster: Address,
}

impl Validation for CanBidOnOwnJob {
    fn validate(&self) -> Result<(), Error> {
        if self.worker == self.job_poster {
            return Err(Error::CannotBidOnOwnJob);
        }
        Ok(())
    }
}

impl CanBidOnOwnJob {
    pub fn create(worker: Address, job_poster: Address) -> Box<CanBidOnOwnJob> {
        Box::new(Self { worker, job_poster })
    }
}
