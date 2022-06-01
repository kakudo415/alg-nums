pub mod addition;
pub mod comparison;
pub mod subtraction;
pub mod utilities;

use std::alloc::alloc;
use std::alloc::Layout;
use std::ptr::copy;
use std::ptr::write_bytes;

use std::ops::Neg;

#[derive(Copy, Clone, PartialEq)]
enum Sign {
    Plus,
    Undefined,
    Minus,
}

pub struct Integer {
    sign: Sign,
    digits: *mut usize,
    length: usize,
    capacity: usize, // TODO: 将来的にはlength==capacityにして一本化？
}

// immutableにすれば、Copyにしてもいい？
impl Integer {
    fn new(capacity: usize) -> Self {
        let new_layout = Layout::array::<usize>(capacity).unwrap();
        let new_ptr = unsafe { alloc(new_layout) } as *mut usize;
        unsafe {
            write_bytes::<usize>(new_ptr, 0, capacity);
        }
        Integer {
            sign: Sign::Undefined,
            digits: new_ptr,
            length: 0,
            capacity: capacity,
        }
    }

    fn fit(mut self) -> Self {
        for i in (0..self.length).rev() {
            if self[i] != 0 {
                break;
            }
            self.length -= 1;
        }
        let mut fitted_integer = Integer::new(self.length);
        fitted_integer.sign = self.sign;
        unsafe {
            copy(self.digits, fitted_integer.digits, self.length);
        }
        fitted_integer.length = self.length;
        fitted_integer
    }

    pub fn zero() -> Self {
        let mut new_zero = Integer::new(1);
        new_zero[0] = 0;
        new_zero
    }

    pub fn abs(&self) -> Self {
        if self.sign == Sign::Minus {
            -self
        } else {
            self.clone()
        }
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        let mut cloned_integer = Integer::new(self.capacity);
        cloned_integer.sign = self.sign;
        unsafe {
            copy(self.digits, cloned_integer.digits, self.length);
        }
        cloned_integer.length = self.length;
        cloned_integer
    }
}

impl Neg for &Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        let mut neg_integer = self.clone();
        neg_integer.sign = match self.sign {
            Sign::Plus => Sign::Minus,
            Sign::Undefined => Sign::Undefined,
            Sign::Minus => Sign::Plus,
        };
        neg_integer
    }
}
