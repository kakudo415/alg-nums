use super::*;

use std::cmp::Ordering::*;
use std::ops::Sub;

impl Sub for &Integer {
    type Output = Integer;

    fn sub(self, other: Self) -> Integer {
        match (self, other) {
            // 0が含まれるもの
            (_, Integer::Zero) => -self,
            (Integer::Zero, _) => other.clone(),
            // 同符号
            (Integer::Plus(lhs), Integer::Plus(rhs)) => match lhs.cmp(rhs) {
                Less => Integer::Minus(rhs - lhs),
                Greater => Integer::Plus(lhs - rhs),
                Equal => Integer::Zero,
            },
            (Integer::Minus(lhs), Integer::Minus(rhs)) => match lhs.cmp(rhs) {
                Greater => Integer::Minus(lhs - rhs),
                Less => Integer::Plus(rhs - lhs),
                Equal => Integer::Zero,
            },
            // 実質足し算
            (Integer::Plus(lhs), Integer::Minus(rhs)) => Integer::Plus(lhs + rhs),
            (Integer::Minus(lhs), Integer::Plus(rhs)) => Integer::Minus(lhs + rhs),
        }
    }
}
