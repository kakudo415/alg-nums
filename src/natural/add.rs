use super::digit::*;
use super::*;

use std::cmp;
use std::ops::Add;

impl Add for &Natural {
    type Output = Natural;

    fn add(self, rhs: Self) -> Natural {
        let mut answer = Natural::new(needed_capacity(self, rhs));

        let mut digit = [0, 0]; // (sum, carried)
        for i in 0..answer.capacity {
            digit = add_carry([self[i], rhs[i], digit[1]]);
            answer[i] = digit[0];
        }
        answer.fit();
        answer
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length) + 1
}
