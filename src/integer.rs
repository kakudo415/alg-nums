pub mod add;
pub mod cmp;
pub mod sub;

// 自然数を拡張して整数を作る
pub use super::natural::*;

use std::fmt;
use std::ops;

#[derive(Copy, Clone, PartialEq)]
enum Sign {
    Plus,
    Undefined,
    Minus,
}

pub struct Integer {
    sign: Sign,
    abs_value: Natural,
}

impl Integer {
    fn new(capacity: usize) -> Self {
        Integer {
            sign: Sign::Undefined,
            abs_value: Natural::new(capacity),
        }
    }

    pub fn zero() -> Self {
        let mut new_zero = Integer::new(1);
        new_zero[0] = 0;
        new_zero
    }

    pub fn abs(&self) -> Self {
        if self.sign == Sign::Minus {
            -self
        } else {
            self.clone()
        }
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        Integer {
            sign: self.sign,
            abs_value: self.abs_value.clone(),
        }
    }
}

impl ops::Neg for &Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        let mut neg_integer = self.clone();
        neg_integer.sign = match self.sign {
            Sign::Plus => Sign::Minus,
            Sign::Undefined => Sign::Undefined,
            Sign::Minus => Sign::Plus,
        };
        neg_integer
    }
}

impl ops::Index<usize> for Integer {
    type Output = usize;
    fn index(&self, idx: usize) -> &usize {
        &self.abs_value[idx]
    }
}

impl ops::IndexMut<usize> for Integer {
    fn index_mut(&mut self, idx: usize) -> &mut usize {
        &mut self.abs_value[idx]
    }
}

impl From<isize> for Integer {
    fn from(value: isize) -> Self {
        if value == 0 {
            return Integer::zero();
        }
        let mut new_integer = Integer::new(1);
        new_integer.sign = if value > 0 { Sign::Plus } else { Sign::Minus };
        new_integer[0] = value.abs() as usize;
        new_integer
    }
}

impl fmt::UpperHex for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.sign == Sign::Minus { "-" } else { " " };
        write!(f, "{}{:X}", sign, self.abs_value)
    }
}
