use std::alloc::{alloc, realloc, Layout};
use std::cmp;
use std::convert::From;
use std::fmt;
use std::ops;
use std::ptr::{write, NonNull};

pub const DIGIT_HALF_BITS: u32 = usize::BITS / 2;
pub const DIGIT_HI: usize = usize::MAX << DIGIT_HALF_BITS;
pub const DIGIT_LO: usize = usize::MAX >> DIGIT_HALF_BITS;

#[derive(Debug)]
pub struct Integer {
    digits: NonNull<usize>,
    length: usize,
}

impl Integer {
    fn _grow(&mut self, old_len: usize, new_len: usize) {
        let old_layout = Layout::array::<usize>(old_len).unwrap();
        let new_ptr = unsafe { realloc(self.digits.as_ptr() as *mut u8, old_layout, new_len) };
        self.digits = NonNull::new(new_ptr as *mut usize).unwrap();
        self.length = new_len;
        for i in old_len..new_len {
            self[i] = 0;
        }
    }

    fn grow(&mut self) {
        self._grow(self.length, self.length * 2);
    }

    // len needs 2^n
    fn grow_into(&mut self, len: usize) {
        if self.length < len {
            self._grow(self.length, len);
        }
    }

    fn new() -> Integer {
        let new_layout = Layout::array::<usize>(1).unwrap();
        let new_ptr = unsafe { alloc(new_layout) };
        Integer {
            digits: NonNull::new(new_ptr as *mut usize).unwrap(),
            length: 1,
        }
    }

    // new()だとどんな値を取っているかが分かりづらいため
    pub fn zero() -> Integer {
        let zero = Self::new();
        unsafe {
            write(zero.digits.as_ptr(), 0);
        }
        zero
    }

    fn carry(&mut self) {
        for i in 1..self.length {
            self[i] += self[i - 1] >> DIGIT_HALF_BITS;
            self[i - 1] &= DIGIT_LO;
        }
        let last = self.length - 1;
        self[last] &= DIGIT_LO;
    }

    pub fn debug_message(&self) -> String {
        let mut debug_info = String::from("[");
        for i in 0..self.length {
            if i == 0 {
                debug_info += format!("{:08X}", self[i]).as_str();
            } else {
                debug_info += format!(" {:08X}", self[i]).as_str();
            }
        }
        debug_info += "]";
        debug_info
    }
}

impl From<usize> for Integer {
    fn from(n: usize) -> Integer {
        let mut new_integer = Self::new();
        if n & DIGIT_HI > 0 {
            new_integer.grow();
            new_integer[0] = n & DIGIT_LO;
            new_integer[1] = n >> DIGIT_HALF_BITS;
        } else {
            new_integer[0] = n;
        }
        new_integer
    }
}

impl ops::Deref for Integer {
    type Target = [usize];

    fn deref(&self) -> &[usize] {
        unsafe { std::slice::from_raw_parts(self.digits.as_ptr(), self.length) }
    }
}

impl ops::DerefMut for Integer {
    fn deref_mut(&mut self) -> &mut [usize] {
        unsafe { std::slice::from_raw_parts_mut(self.digits.as_ptr(), self.length) }
    }
}

impl cmp::PartialEq for Integer {
    fn eq(&self, other: &Integer) -> bool {
        for i in 0..cmp::max(self.length, other.length) {
            let lhs: usize = if i < self.length { self[i] } else { 0 };
            let rhs: usize = if i < other.length { other[i] } else { 0 };
            if lhs != rhs {
                return false;
            }
        }
        true
    }
}

impl cmp::Eq for Integer {}

impl ops::Add for &Integer {
    type Output = Integer;

    fn add(self, other: Self) -> Integer {
        let mut summation = Integer::new();
        let longer_length = cmp::max(self.length, other.length);
        summation.grow_into(longer_length);
        summation.grow();
        for i in 0..longer_length {
            if i >= self.length {
                summation[i] = other[i];
                continue;
            }
            if i >= other.length {
                summation[i] = self[i];
                continue;
            }
            summation[i] = self[i] + other[i];
        }
        summation.carry();
        summation
    }
}
