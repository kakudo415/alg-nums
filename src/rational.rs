mod add;
mod mul;
mod sub;

pub use super::integer::*;

use std::cmp;
use std::fmt;
use std::ops;

pub struct Rational {
    numerator: Integer,
    denominator: Natural,
}

impl ops::Neg for &Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational {
            numerator: -(&self.numerator),
            denominator: self.denominator.clone(),
        }
    }
}

impl From<(Integer, Natural)> for Rational {
    fn from((numerator, denominator): (Integer, Natural)) -> Self {
        Rational {
            numerator: numerator,
            denominator: denominator,
        }
    }
}

impl From<(isize, usize)> for Rational {
    fn from((numerator, denominator): (isize, usize)) -> Self {
        Rational {
            numerator: Integer::from(numerator),
            denominator: Natural::from(denominator),
        }
    }
}

impl fmt::UpperHex for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let numerator = format!("{:X}", self.numerator);
        let denominator = format!("{:X}", self.denominator);
        write!(f, "{}\n", numerator).unwrap();
        for _ in 0..cmp::max(numerator.len() + 1, denominator.len() + 2) {
            write!(f, "─").unwrap();
        }
        write!(f, "\n").unwrap();
        write!(f, " {}", denominator)
    }
}
