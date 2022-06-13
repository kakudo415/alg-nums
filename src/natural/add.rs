use super::digit::add_carry;
use super::Natural;

use std::cmp;
use std::ops::Add;

impl Add for &Natural {
    type Output = Natural;

    fn add(self, rhs: Self) -> Natural {
        let mut answer = Natural::new(needed_capacity(self, rhs));

        let mut sum_carried = [0, 0]; // (sum, carried)
        for i in 0..answer.capacity {
            sum_carried = add_carry([self[i], rhs[i], sum_carried[1]]);
            answer[i] = sum_carried[0];
        }
        answer.normalize();
        answer
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length) + 1
}
