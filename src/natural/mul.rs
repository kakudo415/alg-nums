use super::digit::*;
use super::*;

use std::ops::Mul;

// TODO: ちゃんと計測して閾値を決める
const KARATSUBA_THRESHOLD: usize = 65536;
const FFT_THRESHOLD: usize = 16777216;

impl Mul for &Natural {
    type Output = Natural;

    fn mul(self, other: Self) -> Natural {
        if self.length >= FFT_THRESHOLD && other.length >= FFT_THRESHOLD {
            return fft(self, other);
        }
        if self.length >= KARATSUBA_THRESHOLD && other.length >= KARATSUBA_THRESHOLD {
            return karatsuba(self, other);
        }
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

fn karatsuba(lhs: &Natural, rhs: &Natural) -> Natural {
    let mut answer = Natural::new(needed_capacity(lhs, rhs));
    _karatsuba(&mut answer, lhs, rhs);
    answer
}

fn _karatsuba(answer: &mut Natural, lhs: &Natural, rhs: &Natural) -> Natural {
    todo!()
}

fn fft(lhs: &Natural, rhs: &Natural) -> Natural {
    todo!();
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    lhs.length + rhs.length
}
