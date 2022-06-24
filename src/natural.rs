mod add;
mod cmp;
mod mul;
mod sub;

use super::digit;

use std::alloc;
use std::fmt;
use std::fmt::Write;
use std::ops;
use std::ptr;

// 1, 2, 3, ...
pub struct Natural {
    ptr: *const digit::Digit,
    len: usize,
}

fn longest_natural(digits: &digit::DigitsSlice) -> Option<&digit::DigitsSlice> {
    for i in (0..digits.len()).rev() {
        if digits[i] != 0 {
            return Some(&digits[0..=i]);
        }
    }
    None
}

impl Natural {
    fn from_digits_slice(digits_slice: &digit::DigitsSlice) -> Natural {
        if let Some(digits) = longest_natural(digits_slice) {
            let len = digits.len();
            let layout = alloc::Layout::array::<digit::Digit>(len).unwrap();
            let ptr = unsafe { alloc::alloc_zeroed(layout) } as *mut digit::Digit;
            unsafe {
                ptr::copy(digits_slice.as_ptr(), ptr, len);
            }
            Natural { ptr, len }
        } else {
            panic!("INVALID NATURAL NUMBER");
        }
    }

    fn deref(&self) -> &digit::DigitsSlice {
        unsafe { digit::DigitsSlice::from_raw_parts(self.ptr, self.len) }
    }

    fn clone(&self) -> Self {
        let len = self.len;
        let layout = alloc::Layout::array::<digit::Digit>(len).unwrap();
        let ptr = unsafe { alloc::alloc_zeroed(layout) } as *mut digit::Digit;
        unsafe {
            ptr::copy(self.ptr, ptr, len);
        }
        Natural { ptr, len }
    }

    fn drop(&mut self) {
        let layout = alloc::Layout::array::<digit::Digit>(self.len).unwrap();
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl ops::Deref for Natural {
    type Target = digit::DigitsSlice;
    fn deref(&self) -> &digit::DigitsSlice {
        self.deref()
    }
}

impl From<&digit::DigitsSlice> for Natural {
    fn from(digits_slice: &digit::DigitsSlice) -> Natural {
        Natural::from_digits_slice(digits_slice)
    }
}

impl From<usize> for Natural {
    fn from(n: usize) -> Natural {
        // TODO: n == 0 ERROR
        let mut raw_digits = digit::RawDigits::with_capacity(1);
        raw_digits[0] = n;
        Natural::from_digits_slice(&raw_digits)
    }
}

impl Clone for Natural {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl Drop for Natural {
    fn drop(&mut self) {
        self.drop()
    }
}

impl fmt::UpperHex for Natural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uhex = String::new();
        for i in (0..self.len).rev() {
            write!(uhex, "{:016X}", self[i]).unwrap();
        }
        write!(f, "{}", uhex)
    }
}
