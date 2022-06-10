use super::*;

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
            (Integer::Plus(lhs), Integer::Minus(rhs)) if lhs > rhs => Integer::Plus(lhs - rhs),
            (Integer::Plus(lhs), Integer::Minus(rhs)) if lhs < rhs => Integer::Minus(rhs - lhs),
            (Integer::Plus(lhs), Integer::Minus(rhs)) => Integer::Zero,
            (Integer::Minus(lhs), Integer::Plus(rhs)) if lhs > rhs => Integer::Minus(lhs - rhs),
            (Integer::Minus(lhs), Integer::Plus(rhs)) if lhs < rhs => Integer::Plus(rhs - lhs),
            (Integer::Minus(lhs), Integer::Plus(rhs)) => Integer::Zero,
        }
    }
}
