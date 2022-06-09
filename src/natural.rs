mod add;
mod cmp;
mod digit;
mod mul;
mod sub;

use std::alloc::*;
use std::fmt::*;
use std::ops;
use std::ptr::*;

pub use digit::Digit;

#[derive(Eq, Ord)]
pub struct Natural {
    digits: *mut Digit,
    length: usize,
    capacity: usize,
}

impl Natural {
    pub(crate) fn new(capacity: usize) -> Self {
        let new_layout = Layout::array::<Digit>(capacity).unwrap();
        let new_ptr = unsafe { alloc(new_layout) } as *mut Digit;
        unsafe {
            write_bytes::<Digit>(new_ptr, 0, capacity);
        }
        Natural {
            digits: new_ptr,
            length: 0,
            capacity: capacity,
        }
    }

    fn fit(&mut self) {
        for i in (0..self.length).rev() {
            if self[i] != 0 {
                break;
            }
            self.length -= 1;
        }
    }

    pub fn zero() -> Self {
        let mut new_zero = Natural::new(1);
        new_zero[0] = 0;
        new_zero
    }
}

impl Drop for Natural {
    fn drop(&mut self) {
        let layout = Layout::array::<Digit>(self.capacity).unwrap();
        unsafe {
            dealloc(self.digits as *mut u8, layout);
        }
    }
}

impl Clone for Natural {
    fn clone(&self) -> Self {
        let mut cloned_integer = Natural::new(self.capacity);
        unsafe {
            copy(self.digits, cloned_integer.digits, self.length);
        }
        cloned_integer.length = self.length;
        cloned_integer
    }
}

impl ops::Index<usize> for Natural {
    type Output = Digit;
    fn index(&self, idx: usize) -> &Digit {
        if idx < self.length {
            unsafe { self.digits.offset(idx as isize).as_ref().unwrap() }
        } else {
            &0
        }
    }
}

impl ops::IndexMut<Digit> for Natural {
    fn index_mut(&mut self, idx: usize) -> &mut Digit {
        if idx >= self.capacity {
            panic!("Index(mut) is out of capacity.");
        }
        if idx >= self.length {
            self.length = idx + 1;
        }
        unsafe { self.digits.offset(idx as isize).as_mut().unwrap() }
    }
}

impl From<usize> for Natural {
    fn from(value: usize) -> Self {
        if value == 0 {
            return Natural::zero();
        }
        let mut new_integer = Natural::new(1);
        new_integer[0] = value;
        new_integer
    }
}

impl UpperHex for Natural {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut uhex = String::new();
        for i in (0..self.length).rev() {
            write!(uhex, "{:016X}", self[i]).unwrap();
        }
        write!(f, "{}", uhex)
    }
}
