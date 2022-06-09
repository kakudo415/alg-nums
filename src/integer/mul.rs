use super::*;

use std::ops::Mul;

impl Mul for &Integer {
    type Output = Integer;

    fn mul(self, rhs: Self) -> Integer {
        Integer {
            sign: match (self.sign, rhs.sign) {
                (Sign::Plus, Sign::Plus) | (Sign::Minus, Sign::Minus) => Sign::Plus,
                (Sign::Plus, Sign::Minus) | (Sign::Minus, Sign::Plus) => Sign::Minus,
                (_, _) => Sign::Undefined,
            },
            abs_value: &self.abs_value * &rhs.abs_value,
        }
    }
}
