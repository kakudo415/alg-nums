use super::super::digit;
use super::Natural;

use std::ops::Mul;

impl Mul for &Natural {
    type Output = Natural;

    fn mul(self, other: Self) -> Natural {
        let start_width = next_power_of_2(std::cmp::max(self.len, other.len));
        let mut answer = digit::RawDigits::with_capacity(start_width * 7); // Karatsuba法の場合に備えて大きくとる
        let len = digit::mul::mul(&mut answer, &self, &other, start_width);
        Natural::from(&answer[..len])
    }
}

fn next_power_of_2(mut n: usize) -> usize {
    for i in 0..digit::Digit::BITS {
        if n == 2 << i {
            return n;
        }
    }
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n + 1
}
