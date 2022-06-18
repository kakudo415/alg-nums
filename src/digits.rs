pub mod add;

use std::alloc;
use std::ops;
use std::ptr;

pub(crate) type Digit = usize;

pub(crate) struct Digits {
    pub(crate) pointer: *mut Digit,
    pub(crate) capacity: usize,
}

impl Digits {
    pub(crate) fn new(capacity: usize) -> Digits {
        let layout = alloc::Layout::array::<Digit>(capacity).unwrap();
        let pointer = unsafe { alloc::alloc(layout) } as *mut Digit;
        unsafe {
            ptr::write_bytes::<Digit>(pointer, 0, capacity);
        }
        Digits { pointer, capacity }
    }
}

impl Drop for Digits {
    fn drop(&mut self) {
        let layout = alloc::Layout::array::<Digit>(self.capacity).unwrap();
        unsafe {
            alloc::dealloc(self.pointer as *mut u8, layout);
        }
    }
}

impl ops::Index<usize> for Digits {
    type Output = Digit;
    fn index(&self, idx: usize) -> &Digit {
        if idx >= self.capacity {
            return &0;
        }
        unsafe { self.pointer.offset(idx as isize).as_ref().unwrap() }
    }
}

impl ops::IndexMut<Digit> for Digits {
    fn index_mut(&mut self, idx: usize) -> &mut Digit {
        if idx >= self.capacity {
            panic!("Index(mut) is out of capacity.");
        }
        unsafe { self.pointer.offset(idx as isize).as_mut().unwrap() }
    }
}

const HALF_WIDTH: u32 = Digit::BITS / 2;
const LOW_BITS: Digit = Digit::MAX >> HALF_WIDTH;

// N <= 2^(Digit::BITS / 2)
pub(crate) fn add_carry<const N: usize>(operands: [Digit; N]) -> [Digit; 2] {
    let mut buffer = [0, 0];

    for operand in &operands {
        buffer[0] += operand & LOW_BITS;
        buffer[1] += operand >> HALF_WIDTH;
    }
    buffer[1] += buffer[0] >> HALF_WIDTH;

    buffer[0] = (buffer[0] & LOW_BITS) | (buffer[1] << HALF_WIDTH);
    buffer[1] = buffer[1] >> HALF_WIDTH;

    buffer
}

// 0 < N
pub(crate) fn sub_borrow<const N: usize>(operands: [Digit; N]) -> [Digit; 2] {
    let mut buffer = [operands[0], 0];

    for i in 1..N {
        let operand = operands[i];
        if buffer[0] >= operand {
            buffer[0] -= operand;
        } else {
            buffer[0] = Digit::MAX - operand + buffer[0] + 1;
            buffer[1] += 1;
        }
    }

    buffer
}

pub(crate) fn mul_carry(lhs: Digit, rhs: Digit, carry: Digit, inter: Digit) -> [Digit; 2] {
    let lhs_lo = lhs & LOW_BITS;
    let lhs_hi = lhs >> HALF_WIDTH;
    let rhs_lo = rhs & LOW_BITS;
    let rhs_hi = rhs >> HALF_WIDTH;

    let lo = lhs_lo * rhs_lo; // <= 11100001
    let mid_0 = lhs_lo * rhs_hi; // <= 11100001
    let mid_1 = lhs_hi * rhs_lo; // <= 11100001
    let hi = lhs_hi * rhs_hi; // <= 11100001

    let lo = add_carry([lo, mid_0 << HALF_WIDTH, mid_1 << HALF_WIDTH, carry, inter]); // <= 00000010 11111111
    let hi = hi + (mid_0 >> HALF_WIDTH) + (mid_1 >> HALF_WIDTH) + lo[1]; // <= 11111111

    [lo[0], hi]
}
