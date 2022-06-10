use super::*;

use std::ops::Mul;

impl Mul for &Rational {
    type Output = Rational;

    fn mul(self, other: Self) -> Rational {
        // TODO: 約分
        Rational {
            numerator: &self.numerator * &other.numerator,
            denominator: &self.denominator * &other.denominator,
        }
    }
}
