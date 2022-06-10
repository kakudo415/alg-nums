use super::*;

use std::ops::Add;

impl Add for &Rational {
    type Output = Rational;

    fn add(self, other: Self) -> Rational {
        if self.denominator == other.denominator {
            Rational {
                numerator: &self.numerator + &other.numerator,
                denominator: self.denominator.clone(),
            }
        } else {
            // TODO: GCDをかけるか、後で約分！
            Rational {
                numerator: &(&self.numerator * &Integer::from(&other.denominator))
                    + &(&other.numerator * &Integer::from(&self.denominator)),
                denominator: &self.denominator * &other.denominator,
            }
        }
    }
}
