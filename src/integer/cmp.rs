use super::*;

use std::cmp::*;

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        if self.sign == other.sign && self.abs_value == other.abs_value {
            true
        } else {
            false
        }
    }
}

impl Eq for Integer {}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.sign, other.sign) {
            (Sign::Minus, Sign::Plus)
            | (Sign::Undefined, Sign::Plus)
            | (Sign::Minus, Sign::Undefined) => return Some(Ordering::Less),
            (Sign::Plus, Sign::Minus)
            | (Sign::Plus, Sign::Undefined)
            | (Sign::Undefined, Sign::Minus) => return Some(Ordering::Greater),
            (Sign::Undefined, Sign::Undefined) => return Some(Ordering::Equal),
            (Sign::Plus, Sign::Plus) => return Some(self.abs_value.cmp(&other.abs_value)),
            (Sign::Minus, Sign::Minus) => {
                return Some(self.abs_value.cmp(&other.abs_value).reverse())
            }
        }
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
