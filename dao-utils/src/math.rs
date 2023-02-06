//! Calculation utility functions.
use casper_types::U512;

use crate::Error;

const RATIO_DIVISOR: u32 = 1000;

/// Returns the number divided by 1000.
pub fn to_per_mils<T: Into<U512>>(value: T) -> U512 {
    value.into() / U512::from(RATIO_DIVISOR)
}

/// Multiplies two number and return per mile of their product.
pub fn per_mil_of<T: Into<U512>, R: Into<U512>>(number: T, other: R) -> Result<U512, Error> {
    let number: U512 = number.into();
    let other: U512 = other.into();
    if number < other {
        per_mil_of_u512(other, number)
    } else {
        per_mil_of_u512(number, other)
    }
}

/// Multiplies two number and return per mile of their product casted as u32.
pub fn per_mil_of_as_u32<T: Into<U512>, R: Into<U512>>(number: T, other: R) -> Result<u32, Error> {
    per_mil_of(number, other).and_then(|n| u32::try_from(n).map_err(|_| Error::ArithmeticOverflow))
}

fn per_mil_of_u512(number: U512, other: U512) -> Result<U512, Error> {
    match number.checked_mul(other) {
        // if the result is lower than U512::MAX, divide by the ratio.
        Some(value) => Ok(value / U512::from(RATIO_DIVISOR)),
        // if the result is greater than U512::MAX, do number/ratio * other.
        // It may lead to a precision loss but makes it possible to multiply numbers whose result before the division would be greater that U512::MAX.
        None => (number / U512::from(RATIO_DIVISOR))
            .checked_mul(other)
            .ok_or(Error::ArithmeticOverflow),
    }
}

#[cfg(test)]
mod tests {
    use super::{per_mil_of, U512};
    use crate::Error;
    #[test]
    fn test_per_mils_of() {
        assert_eq!(per_mil_of(1000, 1).unwrap(), 1.into());
        assert_eq!(per_mil_of(1000, 999).unwrap(), 999.into());
        assert_eq!(per_mil_of(6, 334).unwrap(), 2.into());
        assert_eq!(per_mil_of(6, 333).unwrap(), 1.into());
        assert_eq!(per_mil_of(10, 750).unwrap(), 7.into());
        assert_eq!(per_mil_of(U512::MAX, 10).unwrap(), U512::MAX / 100);
        assert_eq!(per_mil_of(10, U512::MAX).unwrap(), U512::MAX / 100);
        assert_eq!(per_mil_of(1001, U512::MAX), Err(Error::ArithmeticOverflow));
    }
}
