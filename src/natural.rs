mod add;
mod cmp;
mod mul;
mod sub;

use super::digits::Digits;

use std::fmt;
use std::fmt::Write;
use std::ops;
use std::ptr;

// 1, 2, 3, ...
// TODO: 非0を保障する
// #[derive(Eq, Ord)]
pub struct Natural {
    digits: Digits,
    length: usize,
}

impl Natural {
    pub(crate) fn new(raw_digits: &Digits) -> Natural {
        let mut length = 0;
        for i in (0..raw_digits.capacity).rev() {
            if raw_digits[i] != 0 {
                break;
            }
        }
        if length == 0 {
            panic!("NATURAL NUMBER CANNOT BE ZERO!");
        }
        Natural {
            digits: *raw_digits,
            length: length,
        }
    }
}

impl Clone for Natural {
    fn clone(&self) -> Self {
        let mut cloned = Natural::new(self.capacity);
        unsafe {
            ptr::copy(self.digits, cloned.digits, self.length);
        }
        cloned.length = self.length;
        cloned
    }
}

impl From<usize> for Natural {
    fn from(value: usize) -> Self {
        if value == 0 {
            panic!("Natural number cannot be zero!");
        }
        let mut new_integer = Natural::new(1);
        new_integer[0] = value;
        new_integer
    }
}

impl fmt::UpperHex for Natural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uhex = String::new();
        for i in (0..self.length).rev() {
            write!(uhex, "{:016X}", self[i]).unwrap();
        }
        write!(f, "{}", uhex)
    }
}
