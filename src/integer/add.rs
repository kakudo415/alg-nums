use super::*;

use std::cmp::Ordering::*;
use std::ops::Add;

impl Add for &Integer {
    type Output = Integer;

    fn add(self, other: Self) -> Integer {
        match (self, other) {
            // 0が含まれるもの
            (_, Integer::Zero) => self.clone(),
            (Integer::Zero, _) => other.clone(),
            // 同符号
            (Integer::Plus(lhs), Integer::Plus(rhs)) => Integer::Plus(lhs + rhs),
            (Integer::Minus(lhs), Integer::Minus(rhs)) => Integer::Minus(lhs + rhs),
            // 実質引き算
            (Integer::Plus(lhs), Integer::Minus(rhs)) => match lhs.cmp(rhs) {
                Less => Integer::Minus(rhs - lhs),
                Greater => Integer::Plus(lhs - rhs),
                Equal => Integer::Zero,
            },
            (Integer::Minus(lhs), Integer::Plus(rhs)) => match lhs.cmp(rhs) {
                Less => Integer::Plus(rhs - lhs),
                Greater => Integer::Minus(lhs - rhs),
                Equal => Integer::Zero,
            },
        }
    }
}
