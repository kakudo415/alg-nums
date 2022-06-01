use super::Integer;
use super::Sign;

use std::cmp::*;

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..max(self.length, other.length) {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Integer {}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Different signs
        match (self.sign, other.sign) {
            (Sign::Minus, Sign::Plus)
            | (Sign::Undefined, Sign::Plus)
            | (Sign::Minus, Sign::Undefined) => return Some(Ordering::Less),
            (Sign::Plus, Sign::Minus)
            | (Sign::Plus, Sign::Undefined)
            | (Sign::Undefined, Sign::Minus) => return Some(Ordering::Greater),
            (Sign::Undefined, Sign::Undefined) => return Some(Ordering::Equal),
            (_, _) => (),
        }

        // (Sign::Plus, Sign::Plus) | (Sign::Minus, Sign::Minus)
        for i in (0..max(self.length, other.length)).rev() {
            if (self.sign == Sign::Plus && self[i] < other[i])
                || (self.sign == Sign::Minus && self[i] > other[i])
            {
                return Some(Ordering::Less);
            }
            if (self.sign == Sign::Plus && self[i] > other[i])
                || (self.sign == Sign::Minus && self[i] < other[i])
            {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }
}
