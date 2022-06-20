use super::digit::*;
use super::Natural;

use std::cmp;
use std::ops::Add;

impl Add for &Natural {
    type Output = Natural;

    fn add(self, other: Self) -> Natural {
        let mut answer = Natural::new(needed_capacity(self, other));
        _add(&mut answer[0..], &self[0..], &other[0..]);
        answer.normalize();
        answer
    }
}

pub(crate) fn _add(answer: &mut [Digit], lhs: &[Digit], rhs: &[Digit]) -> usize {
    let mut sum_carried = [0, 0]; // (sum, carried)
    let mut answer_length = 0;
    for i in 0..answer.len() {
        let ld = if i < lhs.len() { lhs[i] } else { 0 };
        let rd = if i < rhs.len() { rhs[i] } else { 0 };
        sum_carried = add_carry([ld, rd, sum_carried[1]]);
        answer[i] = sum_carried[0];
        answer_length += 1;
        if i >= lhs.len() && i >= rhs.len() && sum_carried[1] == 0 {
            break;
        }
    }
    answer_length
}

pub(crate) fn _add_assign(answer: &mut [Digit], rhs: &[Digit]) -> usize {
    let mut sum_carried = [0, 0]; // (sum, carried)
    let mut answer_length = 0;
    for i in 0..answer.len() {
        let rd = if i < rhs.len() { rhs[i] } else { 0 };
        sum_carried = add_carry([answer[i], rd, sum_carried[1]]);
        answer[i] = sum_carried[0];
        answer_length += 1;
        if i >= rhs.len() && sum_carried[1] == 0 {
            break;
        }
    }
    answer_length
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length) + 1
}
