use casper_types::U512;

use crate::Error;

pub const RATIO_DIVISOR: u32 = 1000;

pub fn per_mil_of<T: Into<U512>, R: Into<U512>>(number: T, other: R) -> Result<U512, Error> {
    let number: U512 = number.into();
    let other: U512 = other.into();
    if number < other {
        per_mil_of_u512(other, number)
    } else {
        per_mil_of_u512(number, other)
    }
}

fn per_mil_of_u512(number: U512, other: U512) -> Result<U512, Error> {
    match number.checked_mul(other) {
        // if the result is lower than U512::MAX, divide by the ratio.
        Some(value) => Ok(value / U512::from(RATIO_DIVISOR)),
        // if the result is greater than U512::MAX, do number/ratio * other.
        // It may lead to a precision loss but makes it possible to multiply numbers whose result before the division would be greater that U512::MAX.
        None => {
            (number / U512::from(RATIO_DIVISOR)).checked_mul(other).ok_or(Error::ArithmeticOverflow)
        }
    }
}

pub fn add_to_balance(current: (bool, U512), amount: U512) -> (bool, U512) {
    let (is_positive, balance) = current;
    if is_positive {
        (true, balance + amount)
    } else if amount < balance {
        (false, balance - amount)
    } else {
        (true, amount - balance)
    }
}

pub fn rem_from_balance(current: (bool, U512), amount: U512) -> (bool, U512) {
    let (is_positive, balance) = current;
    if is_positive || amount.is_zero() && balance.is_zero() {
        if amount <= balance {
            (true, balance - amount)
        } else {
            (false, amount - balance)
        }
    } else {
        (false, balance + amount)
    }
}

#[cfg(test)]
mod tests {
    use super::{add_to_balance, per_mil_of, U512};
    use crate::{math::rem_from_balance, Error};
    #[test]
    fn test_per_mils_of() {
        dbg!(U512::MAX);
        assert_eq!(per_mil_of(1000, 1).unwrap(), 1.into());
        assert_eq!(per_mil_of(1000, 999).unwrap(), 999.into());
        assert_eq!(per_mil_of(6, 334).unwrap(), 2.into());
        assert_eq!(per_mil_of(6, 333).unwrap(), 1.into());
        assert_eq!(per_mil_of(10, 750).unwrap(), 7.into());
        assert_eq!(per_mil_of(U512::MAX, 10).unwrap(), U512::MAX / 100);
        assert_eq!(per_mil_of(10, U512::MAX).unwrap(), U512::MAX / 100);
        assert_eq!(per_mil_of(1001, U512::MAX), Err(Error::ArithmeticOverflow));  
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_balance_math() {
        let ZERO = U512::zero();
        let ONE = U512::one();
        let TWO = ONE + ONE;

        let P_ZERO = (true, ZERO);
        let P_ONE = (true, ONE);
        let P_TWO = (true, TWO);

        let N_ZERO = (false, ZERO);
        let N_ONE = (false, ONE);
        let N_TWO = (false, TWO);

        // 0 + 0 == 0
        assert_eq!(add_to_balance(P_ZERO, ZERO), P_ZERO);
        assert_eq!(add_to_balance(N_ZERO, ZERO), P_ZERO);

        // 0 + 1 == 1
        assert_eq!(add_to_balance(P_ONE, ZERO), P_ONE);

        // 1 + 1 == 1
        assert_eq!(add_to_balance(P_ONE, ONE), P_TWO);

        // -2 + 1 == -1
        assert_eq!(add_to_balance(N_TWO, ONE), N_ONE);

        // -1 + 1 == 0
        assert_eq!(add_to_balance(N_ONE, ONE), P_ZERO);

        // -1 + 2 == 1
        assert_eq!(add_to_balance(N_ONE, TWO), P_ONE);

        // 0 + 0 == 0
        assert_eq!(rem_from_balance(P_ZERO, ZERO), P_ZERO);
        assert_eq!(rem_from_balance(N_ZERO, ZERO), P_ZERO);

        // 0 - 1 == -1
        assert_eq!(rem_from_balance(N_ZERO, ONE), N_ONE);

        // -1 - 1 == -2
        assert_eq!(rem_from_balance(N_ONE, ONE), N_TWO);

        // 2 - 1 == 1
        assert_eq!(rem_from_balance(P_TWO, ONE), P_ONE);

        // 1 - 2 == -1
        assert_eq!(rem_from_balance(P_ONE, TWO), N_ONE);

        // 2 - 2 == 0
        assert_eq!(rem_from_balance(P_TWO, TWO), P_ZERO);
    }
}
