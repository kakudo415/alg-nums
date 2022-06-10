use super::*;

use std::ops::Mul;

impl Mul for &Integer {
    type Output = Integer;

    fn mul(self, other: Self) -> Integer {
        match (self, other) {
            (Integer::Plus(lhs), Integer::Plus(rhs)) | (Integer::Minus(lhs), Integer::Minus(rhs)) => Integer::Plus(lhs * rhs),
            (Integer::Plus(lhs), Integer::Minus(rhs)) | (Integer::Minus(lhs), Integer::Plus(rhs)) => Integer::Minus(lhs * rhs),
            (_, _) => Integer::Zero
        }
    }
}
