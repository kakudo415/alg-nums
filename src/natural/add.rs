use super::Natural;
use crate::digits::add::partial_add;
use crate::digits::Digits;

use std::cmp;
use std::ops::Add;

impl Add for &Natural {
    type Output = Natural;

    fn add(self, other: Self) -> Natural {
        let mut buffer = Digits::new(needed_capacity(self, other));
        partial_add(
            &mut buffer,
            0,
            &self.digits,
            (0, self.length),
            &other.digits,
            (0, other.length),
        );
        Natural::new(&buffer)
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length) + 1
}
