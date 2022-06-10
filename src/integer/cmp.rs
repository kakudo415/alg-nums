use super::*;

use std::cmp::*;

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Integer::Zero, Integer::Zero) => true,
            (Integer::Plus(lhs), Integer::Plus(rhs)) if lhs == rhs => true,
            (Integer::Minus(lhs), Integer::Minus(rhs)) if lhs == rhs => true,
            (_, _) => false,
        }
    }
}

impl Eq for Integer {}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Integer::Zero, Integer::Zero) => Some(Ordering::Equal),
            (Integer::Minus(_), Integer::Zero)
            | (Integer::Zero, Integer::Plus(_))
            | (Integer::Minus(_), Integer::Plus(_)) => Some(Ordering::Less),
            (Integer::Plus(_), Integer::Zero)
            | (Integer::Zero, Integer::Minus(_))
            | (Integer::Plus(_), Integer::Minus(_)) => Some(Ordering::Greater),
            (Integer::Plus(lhs), Integer::Plus(rhs)) => Some(lhs.cmp(rhs)),
            (Integer::Minus(lhs), Integer::Minus(rhs)) => Some(lhs.cmp(rhs).reverse()),
        }
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
