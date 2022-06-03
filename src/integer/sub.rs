use super::*;

use std::ops::Sub;

impl Sub for &Integer {
    type Output = Integer;

    fn sub(self, rhs: Self) -> Integer {
        self + &(-rhs)
    }
}
