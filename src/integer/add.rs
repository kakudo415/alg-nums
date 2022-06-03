use super::*;

use std::ops::Add;

impl Add for &Integer {
    type Output = Integer;

    fn add(self, rhs: Self) -> Integer {
        match (self.sign, rhs.sign) {
            // z + 0, 0 + z
            (_, Sign::Undefined) => self.clone(),
            (Sign::Undefined, _) => rhs.clone(),
            // Same signs
            (Sign::Plus, Sign::Plus) | (Sign::Minus, Sign::Minus) => Integer {
                sign: self.sign,
                abs_value: &self.abs_value + &rhs.abs_value,
            },
            // l + (-r) = l - r
            (Sign::Plus, Sign::Minus) if self > rhs => Integer {
                sign: Sign::Plus,
                abs_value: &self.abs_value - &rhs.abs_value,
            },
            (Sign::Plus, Sign::Minus) if self < rhs => Integer {
                sign: Sign::Minus,
                abs_value: &self.abs_value - &rhs.abs_value,
            },
            (Sign::Plus, Sign::Minus) => Integer {
                sign: Sign::Undefined,
                abs_value: Natural::zero(),
            },
            // (-l) + r = r - l
            (Sign::Minus, Sign::Plus) if self > rhs => Integer {
                sign: Sign::Minus,
                abs_value: &rhs.abs_value - &self.abs_value,
            },
            (Sign::Minus, Sign::Plus) if self < rhs => Integer {
                sign: Sign::Plus,
                abs_value: &rhs.abs_value - &self.abs_value,
            },
            (Sign::Minus, Sign::Plus) => Integer {
                sign: Sign::Undefined,
                abs_value: Natural::zero(),
            },
        }
    }
}
