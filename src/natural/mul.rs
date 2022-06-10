use super::digit::*;
use super::*;

use std::ops::Mul;

impl Mul for &Natural {
    type Output = Natural;

    // TODO: Karatsuba法・FFT法も実装する
    fn mul(self, other: Self) -> Natural {
        school(self, other)
    }
}

fn school(lhs: &Natural, rhs: &Natural) -> Natural {
    let mut answer = Natural::new(needed_capacity(lhs, rhs));

    for i in 0..rhs.length {
        if rhs[i] == 0 {
            continue;
        }
        let mut internal_answer = [0, 0];
        for j in 0..(lhs.length + 1) {
            internal_answer = mul_carry(rhs[i], lhs[j], internal_answer[1], answer[i + j]);
            answer[i + j] = internal_answer[0];
        }
    }

    answer.normalize();
    answer
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    lhs.length + rhs.length
}
