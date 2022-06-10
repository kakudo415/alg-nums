use super::digit::*;
use super::*;

use std::ops::Mul;

impl Mul for &Natural {
    type Output = Natural;

    // TODO: Karatsuba法・FFT法も実装する
    fn mul(self, rhs: Self) -> Natural {
        let mut answer = Natural::new(needed_capacity(self, rhs));

        for i in 0..rhs.length {
            if rhs[i] == 0 {
                continue;
            }
            answer = &answer + &mul_1digit(self, rhs[i], i);
        }

        answer.normalize();
        answer
    }
}

#[inline]
fn mul_1digit(lhs: &Natural, rhs: Digit, offset: Digit) -> Natural {
    let mut answer = Natural::new(offset + lhs.length + 1);

    let mut digit = [0, 0];
    for i in 0..lhs.capacity {
        digit = mul_carry(lhs[i], rhs, digit[1]);
        answer[offset + i] = digit[0];
    }

    answer.normalize();
    answer
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    lhs.length + rhs.length // TODO: もうちょっと賢くする
}
