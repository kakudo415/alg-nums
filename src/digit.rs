pub mod calc;

pub mod add;
pub mod mul;
pub mod sub;

use std::alloc;
use std::ops;

pub type Digit = usize;

pub struct RawDigits {
    ptr: *mut Digit,
    cap: usize,
}

pub struct DigitsSlice {
    ptr: *mut Digit,
    len: usize,
}

impl RawDigits {
    pub fn with_capacity(cap: usize) -> RawDigits {
        let layout = alloc::Layout::array::<Digit>(cap).unwrap();
        let ptr = unsafe { alloc::alloc_zeroed(layout) } as *mut Digit;
        RawDigits { ptr, cap }
    }

    pub fn as_ptr(&self) -> *const Digit {
        self.ptr
    }
    pub fn as_mut_ptr(&mut self) -> *mut Digit {
        self.ptr
    }

    pub fn deref(&self) -> &DigitsSlice {
        let layout = alloc::Layout::new::<DigitsSlice>();
        unsafe {
            let digits_slice = alloc::alloc(layout) as *mut DigitsSlice;
            (*digits_slice).ptr = self.ptr;
            (*digits_slice).len = self.cap;
            &*digits_slice
        }
    }
    pub fn deref_mut(&mut self) -> &mut DigitsSlice {
        let layout = alloc::Layout::new::<DigitsSlice>();
        unsafe {
            let digits_slice = alloc::alloc(layout) as *mut DigitsSlice;
            (*digits_slice).ptr = self.ptr;
            (*digits_slice).len = self.cap;
            &mut *digits_slice
        }
    }

    fn drop(&mut self) {
        let layout = alloc::Layout::array::<Digit>(self.cap).unwrap();
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl DigitsSlice {
    pub unsafe fn from_raw_parts(ptr: *const Digit, len: usize) -> &'static Self {
        let layout = alloc::Layout::new::<DigitsSlice>();
        let digits_slice = alloc::alloc(layout) as *mut DigitsSlice;
        (*digits_slice).ptr = ptr as *mut Digit;
        (*digits_slice).len = len;
        &*digits_slice
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_ptr(&self) -> *const Digit {
        self.ptr
    }
    pub fn as_mut_ptr(&mut self) -> *mut Digit {
        self.ptr
    }

    fn index(&self, idx: usize) -> &Digit {
        if idx >= self.len {
            return &0;
        }
        unsafe { self.ptr.offset(idx as isize).as_ref().unwrap() }
    }
    fn index_mut(&mut self, idx: usize) -> &mut Digit {
        if idx >= self.len {
            panic!("INDEX_MUT FOR DIGITS SLICE IS OUT OF RANGE");
        }
        unsafe { self.ptr.offset(idx as isize).as_mut().unwrap() }
    }

    fn index_range(&self, rng: ops::Range<usize>) -> &DigitsSlice {
        let head = &self[rng.start] as *const Digit;
        unsafe { DigitsSlice::from_raw_parts(head, rng.end - rng.start) }
    }

    fn index_range_inclusive(&self, rng: ops::RangeInclusive<usize>) -> &DigitsSlice {
        let head = &self[*rng.start()] as *const Digit;
        unsafe { DigitsSlice::from_raw_parts(head, *rng.end() - *rng.start() + 1) }
    }
}

impl ops::Deref for RawDigits {
    type Target = DigitsSlice;
    fn deref(&self) -> &DigitsSlice {
        self.deref()
    }
}
impl ops::DerefMut for RawDigits {
    fn deref_mut(&mut self) -> &mut DigitsSlice {
        self.deref_mut()
    }
}

impl Drop for RawDigits {
    fn drop(&mut self) {
        self.drop()
    }
}

impl ops::Index<usize> for DigitsSlice {
    type Output = Digit;
    fn index(&self, idx: usize) -> &Digit {
        self.index(idx)
    }
}
impl ops::IndexMut<usize> for DigitsSlice {
    fn index_mut(&mut self, idx: usize) -> &mut Digit {
        self.index_mut(idx)
    }
}

impl ops::Index<ops::Range<usize>> for DigitsSlice {
    type Output = DigitsSlice;
    fn index(&self, rng: ops::Range<usize>) -> &DigitsSlice {
        self.index_range(rng)
    }
}

impl ops::Index<ops::RangeInclusive<usize>> for DigitsSlice {
    type Output = DigitsSlice;
    fn index(&self, rng: ops::RangeInclusive<usize>) -> &DigitsSlice {
        self.index_range_inclusive(rng)
    }
}
