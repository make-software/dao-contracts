use crate::conversions::BytesConversion;
use crate::Error;
use casper_types::{U256, U512};

pub const RATIO_DIVISOR: u32 = 1000;

pub fn promils_of(number: U256, promils: U256) -> Result<U256, Error> {
    let number_u512 = U512::convert_from_bytes(number.convert_to_bytes()?)?;
    let ratio_u512 = U512::convert_from_bytes(promils.convert_to_bytes()?)?;

    let dividend = number_u512 * ratio_u512;

    let result = dividend / U512::from(RATIO_DIVISOR);

    if result > U512::convert_from_bytes(U256::MAX.convert_to_bytes()?)? {
        Err(Error::ArithmeticOverflow)
    } else {
        Ok(U256::convert_from_bytes(result.convert_to_bytes()?)?)
    }
}
