pub mod addition;
pub mod comparison;
pub mod utilities;

use std::alloc::alloc;
use std::alloc::Layout;
use std::ptr::write_bytes;

// pub const DIGIT_HALF_BITS: u32 = usize::BITS / 2;
// pub const DIGIT_HI: usize = usize::MAX << DIGIT_HALF_BITS;
// pub const DIGIT_LO: usize = usize::MAX >> DIGIT_HALF_BITS;

pub struct Integer {
    // TODO: Sign
    digits: *mut usize,
    length: usize,
    capacity: usize,
}

impl Integer {
    fn new(capacity: usize) -> Self {
        let new_layout = Layout::array::<usize>(capacity).unwrap();
        let new_ptr = unsafe { alloc(new_layout) } as *mut usize;
        unsafe {
            write_bytes::<usize>(new_ptr, 0, capacity);
        }
        Integer {
            digits: new_ptr,
            length: 0,
            capacity: capacity,
        }
    }

    pub fn zero() -> Self {
        let mut new_zero = Integer::new(1);
        new_zero[0] = 0;
        new_zero
    }

    pub fn one() -> Self {
        let mut new_one = Integer::new(1);
        new_one[0] = 1;
        new_one
    }
}
