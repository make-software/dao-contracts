use casper_types::U512;

use crate::{conversions::BytesConversion, Error};

pub const RATIO_DIVISOR: u32 = 1000;

// TODO: Refactor this!
pub fn promils_of(number: U512, promils: U512) -> Result<U512, Error> {
    let number_u512 = U512::convert_from_bytes(number.convert_to_bytes()?)?;
    let ratio_u512 = U512::convert_from_bytes(promils.convert_to_bytes()?)?;

    let dividend = number_u512 * ratio_u512;

    let result = dividend / U512::from(RATIO_DIVISOR);

    if result > U512::convert_from_bytes(U512::MAX.convert_to_bytes()?)? {
        Err(Error::ArithmeticOverflow)
    } else {
        Ok(U512::convert_from_bytes(result.convert_to_bytes()?)?)
    }
}

// TODO: Refactor this even more!
pub fn promils_of_u512(number: U512, promils: U512) -> Result<U512, Error> {
    let number_u512 = number;
    let ratio_u512 = promils;

    let dividend = number_u512 * ratio_u512;

    let result = dividend / U512::from(RATIO_DIVISOR);

    if result > U512::convert_from_bytes(U512::MAX.convert_to_bytes()?)? {
        Err(Error::ArithmeticOverflow)
    } else {
        Ok(U512::convert_from_bytes(result.convert_to_bytes()?)?)
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
    use super::{add_to_balance, promils_of, U512};
    use crate::math::rem_from_balance;
    #[test]
    fn test_promils_of() {
        assert_eq!(promils_of(1000.into(), 1.into()).unwrap(), 1.into());
        assert_eq!(promils_of(1000.into(), 999.into()).unwrap(), 999.into());
        assert_eq!(promils_of(6.into(), 334.into()).unwrap(), 2.into());
        assert_eq!(promils_of(6.into(), 333.into()).unwrap(), 1.into());
        assert_eq!(promils_of(10.into(), 750.into()).unwrap(), 7.into());
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
