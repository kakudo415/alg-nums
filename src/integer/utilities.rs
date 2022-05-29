use super::Integer;

use std::fmt;
use std::fmt::Write;
use std::ops;

impl ops::Index<usize> for Integer {
    type Output = usize;
    fn index(&self, idx: usize) -> &usize {
        if idx < self.length {
            unsafe { self.digits.offset(idx as isize).as_ref().unwrap() }
        } else {
            &0
        }
    }
}

impl ops::IndexMut<usize> for Integer {
    fn index_mut(&mut self, idx: usize) -> &mut usize {
        if idx >= self.capacity {
            panic!("Index(mut) is out of capacity.");
        }
        if idx >= self.length {
            self.length = idx + 1;
        }
        unsafe { self.digits.offset(idx as isize).as_mut().unwrap() }
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        if value == 0 {
            return Integer::zero();
        }
        let mut new_integer = Integer::new(1);
        new_integer[0] = value;
        new_integer
    }
}

impl fmt::UpperHex for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uhex = String::new();
        for i in (0..self.length).rev() {
            write!(uhex, "{:016X}", self[i]).unwrap();
        }
        write!(f, "{}", uhex)
    }
}
