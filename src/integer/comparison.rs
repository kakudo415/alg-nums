use super::Integer;

use std::cmp;

impl cmp::PartialEq for Integer {
    fn eq(&self, other: &Integer) -> bool {
        for i in 0..cmp::max(self.length, other.length) {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl cmp::Eq for Integer {}
