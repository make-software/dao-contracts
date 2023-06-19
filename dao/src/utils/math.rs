//! Calculation utility functions.
use crate::utils::Error;
use odra::types::Balance;

const RATIO_DIVISOR: u32 = 1000;

/// Returns the number divided by 1000.
pub fn to_per_mils<T: Into<Balance>>(value: T) -> Balance {
    value.into() / Balance::from(RATIO_DIVISOR)
}

/// Multiplies two number and return per mile of their product.
pub fn per_mil_of<T: Into<Balance>, R: Into<Balance>>(
    number: T,
    other: R,
) -> Result<Balance, Error> {
    let number: Balance = number.into();
    let other: Balance = other.into();
    if number < other {
        per_mil_of_ordered(other, number)
    } else {
        per_mil_of_ordered(number, other)
    }
}

/// Multiplies two number and return per mile of their product casted as u32.
pub fn per_mil_of_as_u32<T: Into<Balance>, R: Into<Balance>>(
    number: T,
    other: R,
) -> Result<u32, Error> {
    per_mil_of(number, other).and_then(|n| u32::try_from(n).map_err(|_| Error::ArithmeticOverflow))
}

fn per_mil_of_ordered(number: Balance, other: Balance) -> Result<Balance, Error> {
    match number.checked_mul(other) {
        // if the result is lower than Balance::MAX, divide by the ratio.
        Some(value) => Ok(value / Balance::from(RATIO_DIVISOR)),
        // if the result is greater than Balance::MAX, do number/ratio * other.
        // It may lead to a precision loss but makes it possible to multiply numbers whose result before the division would be greater that Balance::MAX.
        None => (number / Balance::from(RATIO_DIVISOR))
            .checked_mul(other)
            .ok_or(Error::ArithmeticOverflow),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_mils_of() {
        assert_eq!(per_mil_of(1000, 1).unwrap(), 1.into());
        assert_eq!(per_mil_of(1000, 999).unwrap(), 999.into());
        assert_eq!(per_mil_of(6, 334).unwrap(), 2.into());
        assert_eq!(per_mil_of(6, 333).unwrap(), 1.into());
        assert_eq!(per_mil_of(10, 750).unwrap(), 7.into());
    }
}
