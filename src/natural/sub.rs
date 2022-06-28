use super::super::digit;
use super::Natural;

use std::cmp;
use std::ops::Sub;

impl Sub for &Natural {
    type Output = Natural;

    fn sub(self, other: Self) -> Natural {
        let mut answer = digit::RawDigits::with_capacity(needed_capacity(self, other));
        digit::sub::sub(&mut answer, &self, &other);
        Natural::from(answer.deref()) // FIXME: なんでDerefされへんの？ again.
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.len, rhs.len)
}
