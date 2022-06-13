mod add;
mod cmp;
mod digit;
mod mul;
mod sub;

use digit::Digit;

use std::alloc;
use std::fmt;
use std::fmt::Write;
use std::ops;
use std::ptr;

// 1, 2, 3, ...
// TODO: 非0を保障する
#[derive(Eq, Ord)]
pub struct Natural {
    digits: *mut Digit,
    length: usize,
    capacity: usize,
}

impl Natural {
    pub(crate) fn new(capacity: usize) -> Self {
        let layout = alloc::Layout::array::<Digit>(capacity).unwrap();
        let pointer = unsafe { alloc::alloc(layout) } as *mut Digit;
        unsafe {
            ptr::write_bytes::<Digit>(pointer, 0, capacity);
        }
        Natural {
            digits: pointer,
            length: 0,
            capacity: capacity,
        }
    }

    fn normalize(&mut self) {
        for i in (0..self.length).rev() {
            if self[i] != 0 {
                break;
            }
            self.length -= 1;
        }
        if self.length == 0 {
            panic!("NATURAL NUMBER CANNOT BE ZERO!");
        }
    }
}

impl Drop for Natural {
    fn drop(&mut self) {
        let layout = alloc::Layout::array::<Digit>(self.capacity).unwrap();
        unsafe {
            alloc::dealloc(self.digits as *mut u8, layout);
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
