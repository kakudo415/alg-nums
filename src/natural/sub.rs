use super::digit::*;
use super::Natural;

use std::cmp;
use std::ops::Sub;

impl Sub for &Natural {
    type Output = Natural;

    fn sub(self, other: Self) -> Natural {
        let mut answer = Natural::new(needed_capacity(self, other));
        _sub(&mut answer[0..], &self[0..], &other[0..]);
        answer.normalize();
        answer
    }
}

pub(crate) fn _sub(answer: &mut [Digit], lhs: &[Digit], rhs: &[Digit]) {
    let mut digit = [0, 0]; // (diff, borrow)
    for i in 0..answer.len() {
        let ld = if i < lhs.len() { lhs[i] } else { 0 };
        let rd = if i < rhs.len() { rhs[i] } else { 0 };
        digit = sub_borrow([ld, rd, digit[1]]);
        answer[i] = digit[0];
    }
    if digit[1] > 0 {
        panic!("Cannot calculate (LITTLE - BIG) in Natural Number");
    }
}

pub(crate) fn _sub_assign(answer: &mut [Digit], rhs: &[Digit]) {
    let mut digit = [0, 0]; // (diff, borrow)
    for i in 0..answer.len() {
        let rd = if i < rhs.len() { rhs[i] } else { 0 };
        digit = sub_borrow([answer[i], rd, digit[1]]);
        answer[i] = digit[0];
    }
    if digit[1] > 0 {
        panic!("Cannot calculate (LITTLE - BIG) in Natural Number");
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length)
}
