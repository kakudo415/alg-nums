use super::*;

use std::ops::Sub;

impl Sub for &Integer {
    type Output = Integer;

    fn sub(self, other: Self) -> Integer {
        self + &(-other)
    }
}
