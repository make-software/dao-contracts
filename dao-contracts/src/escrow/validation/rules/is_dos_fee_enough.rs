use std::rc::Rc;

use casper_dao_utils::Error;
use casper_types::U512;

use crate::{rules::validation::Validation, Configuration};

pub struct IsDosFeeEnough {
    pub configuration: Rc<Configuration>,
    pub dos_fee: U512,
}

impl Validation for IsDosFeeEnough {
    fn validate(&self) -> Result<(), Error> {
        let fiat_value = self.configuration.convert_to_fiat(self.dos_fee)?;
        if self.configuration.is_post_job_dos_fee_too_low(fiat_value) {
            return Err(Error::DosFeeTooLow);
        };

        Ok(())
    }
}

impl IsDosFeeEnough {
    pub fn create(configuration: Rc<Configuration>, dos_fee: U512) -> Box<IsDosFeeEnough> {
        Box::new(Self {
            configuration,
            dos_fee,
        })
    }
}
