use std::alloc::{alloc, realloc, Layout};
use std::convert::From;
use std::fmt;
use std::ptr::{write, NonNull};

pub struct Integer {
    dgt: NonNull<usize>,
    len: usize,
}

impl Integer {
    fn _grow(&mut self, old_len: usize, new_len: usize) {
        let old_layout = Layout::array::<usize>(old_len).unwrap();
        let new_ptr = unsafe { realloc(self.dgt.as_ptr() as *mut u8, old_layout, new_len) };
        self.dgt = NonNull::new(new_ptr as *mut usize).unwrap();
    }

    fn grow(&mut self) {
        self._grow(self.len, self.len * 2);
    }

    // len needs 2^n
    fn grow_into(&mut self, len: usize) {
        if self.len < len {
            self._grow(self.len, len);
        }
    }

    fn new() -> Integer {
        let new_layout = Layout::array::<usize>(1).unwrap();
        let new_ptr = unsafe { alloc(new_layout) };
        Integer {
            dgt: NonNull::new(new_ptr as *mut usize).unwrap(),
            len: 1,
        }
    }

    // new()だとどんな値を取っているかが分かりづらいため
    pub fn zero() -> Integer {
        let to_be_zero = Self::new();
        unsafe {
            write(to_be_zero.dgt.as_ptr(), 0);
        }
        to_be_zero
    }
}

impl From<usize> for Integer {
    fn from(n: usize) -> Integer {
        let i = Self::new();
        unsafe {
            write(i.dgt.as_ptr(), n);
        }
        i
    }
}

// TODO: 多バイト対応する
impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe { self.dgt.as_ref() })
    }
}
