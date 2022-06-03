pub mod add;
pub mod cmp;
pub mod misc;
pub mod sub;

// 自然数を拡張して整数を作る
pub mod natural;
pub use natural::*;

use std::ops::Neg;

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

// immutableにすれば、Copyにしてもいい？
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

impl Neg for &Integer {
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
