pub mod add;
pub mod cmp;
pub mod mul;
pub mod sub;

// 自然数を拡張して整数を作る
pub use super::natural::*;

use std::fmt;
use std::ops;

// ..., -2, -1, 0, 1, 2, ...
pub enum Integer {
    Zero,
    Plus(Natural),
    Minus(Natural),
}

impl Integer {
    pub fn zero() -> Self {
        Integer::Zero
    }

    pub fn abs(&self) -> Self {
        match self {
            Integer::Zero => Integer::Zero,
            Integer::Plus(value) => Integer::Plus(value.clone()),
            Integer::Minus(value) => Integer::Plus(value.clone()),
        }
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        match self {
            Integer::Zero => Integer::Zero,
            Integer::Plus(value) => Integer::Plus(value.clone()),
            Integer::Minus(value) => Integer::Minus(value.clone()),
        }
    }
}

impl ops::Neg for &Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        match self {
            Integer::Zero => Integer::Zero,
            Integer::Plus(value) => Integer::Minus(value.clone()),
            Integer::Minus(value) => Integer::Plus(value.clone()),
        }
    }
}

impl From<&Natural> for Integer {
    fn from(value: &Natural) -> Self {
        Integer::Plus(value.clone())
    }
}

impl From<isize> for Integer {
    fn from(value: isize) -> Self {
        if value == 0 {
            return Integer::zero();
        }
        if value > 0 {
            Integer::Plus(Natural::from(value.abs() as usize))
        } else {
            Integer::Minus(Natural::from(value.abs() as usize))
        }
    }
}

impl fmt::UpperHex for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Integer::Zero => write!(f, " 0"),
            Integer::Plus(value) => write!(f, " {:X}", value),
            Integer::Minus(value) => write!(f, "-{:X}", value),
        }
    }
}
