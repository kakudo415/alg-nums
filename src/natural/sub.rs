use super::digit::*;
use super::*;

use std::cmp;
use std::ops::Sub;

impl Sub for &Natural {
    type Output = Natural;

    fn sub(self, rhs: Self) -> Natural {
        let mut answer = Natural::new(needed_capacity(self, rhs));

        let mut digit = [0, 0]; // (diff, borrow)
        for i in 0..answer.capacity {
            digit = sub_borrow([self[i], rhs[i], digit[1]]);
            answer[i] = digit[0];
        }
        if digit[1] > 0 {
            panic!("Cannot calculate (LITTLE - BIG) in Natural Number");
        }
        answer.normalize();
        answer
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length)
}
