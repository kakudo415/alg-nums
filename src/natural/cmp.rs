use super::*;

use std::cmp::*;

impl PartialEq for Natural {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..max(self.length, other.length) {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in (0..max(self.length, other.length)).rev() {
            if self[i] < other[i] {
                return Some(Ordering::Less);
            }
            if self[i] > other[i] {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }
}
