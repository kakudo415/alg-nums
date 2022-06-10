use super::*;

use std::ops::Sub;

impl Sub for &Rational {
    type Output = Rational;

    fn sub(self, other: Self) -> Rational {
        self + &(-other)
    }
}
