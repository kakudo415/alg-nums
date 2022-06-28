use super::Natural;

use std::cmp;
use std::cmp::Ordering;

impl PartialEq for Natural {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        for i in 0..cmp::max(self.len, other.len) {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Natural {}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in (0..cmp::max(self.len, other.len)).rev() {
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

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
