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

pub struct Digits {
    ptr: *mut Digit,
    len: usize,
    rlen: isize, // Real length
}

impl RawDigits {
    pub fn with_capacity(cap: usize) -> RawDigits {
        let layout = alloc::Layout::array::<Digit>(cap).unwrap();
        let ptr = unsafe { alloc::alloc_zeroed(layout) } as *mut Digit;
        RawDigits { ptr, cap }
    }

    #[allow(dead_code)]
    pub fn as_ptr(&self) -> *const Digit {
        self.ptr
    }
    #[allow(dead_code)]
    pub fn as_mut_ptr(&mut self) -> *mut Digit {
        self.ptr
    }

    pub fn deref(&self) -> &Digits {
        let layout = alloc::Layout::new::<Digits>();
        unsafe {
            let digits = alloc::alloc(layout) as *mut Digits;
            (*digits).ptr = self.ptr;
            (*digits).len = self.cap;
            (*digits).rlen = self.cap as isize;
            &*digits
        }
    }
    pub fn deref_mut(&mut self) -> &mut Digits {
        let layout = alloc::Layout::new::<Digits>();
        unsafe {
            let digits = alloc::alloc(layout) as *mut Digits;
            (*digits).ptr = self.ptr;
            (*digits).len = self.cap;
            (*digits).rlen = self.cap as isize;
            &mut *digits
        }
    }

    fn drop(&mut self) {
        let layout = alloc::Layout::array::<Digit>(self.cap).unwrap();
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl Digits {
    // TODO: このライフタイムは何を表しているのか調べる
    pub unsafe fn from_raw_parts<'a>(ptr: *const Digit, len: usize, rlen: isize) -> &'a Self {
        let layout = alloc::Layout::new::<Digits>();
        let digits = alloc::alloc(layout) as *mut Digits;
        (*digits).ptr = ptr as *mut Digit;
        (*digits).len = len;
        (*digits).rlen = rlen;
        &*digits
    }

    pub unsafe fn from_raw_parts_mut<'a>(ptr: *mut Digit, len: usize, rlen: isize) -> &'a mut Self {
        let layout = alloc::Layout::new::<Digits>();
        let digits = alloc::alloc(layout) as *mut Digits;
        (*digits).ptr = ptr as *mut Digit;
        (*digits).len = len;
        (*digits).rlen = rlen;
        &mut *digits
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
        if idx as isize >= self.rlen || idx >= self.len {
            return &0;
        }
        unsafe { self.ptr.offset(idx as isize).as_ref().unwrap() }
    }
    fn index_mut(&mut self, idx: usize) -> &mut Digit {
        if idx as isize >= self.rlen || idx >= self.len {
            panic!("INDEX_MUT FOR DIGITS IS OUT OF RANGE");
        }
        unsafe { self.ptr.offset(idx as isize).as_mut().unwrap() }
    }

    fn index_range(&self, rng: ops::Range<usize>) -> &Digits {
        let head = &self[rng.start] as *const Digit;
        unsafe {
            Digits::from_raw_parts(
                head,
                rng.end - rng.start,
                std::cmp::min(self.rlen as isize - rng.start as isize, self.len as isize - rng.start as isize),
            )
        }
    }
    fn index_range_mut(&mut self, rng: ops::Range<usize>) -> &mut Digits {
        let head = &mut self[rng.start] as *mut Digit;
        unsafe {
            Digits::from_raw_parts_mut(
                head,
                rng.end - rng.start,
                std::cmp::min(self.rlen as isize - rng.start as isize, self.len as isize - rng.start as isize),
            )
        }
    }

    fn index_range_inclusive(&self, rng: ops::RangeInclusive<usize>) -> &Digits {
        self.index_range(*rng.start()..*rng.end() + 1)
        // let head = &self[*rng.start()] as *const Digit;
        // unsafe { Digits::from_raw_parts(head, *rng.end() - *rng.start() + 1) }
    }

    fn index_range_from(&self, rng: ops::RangeFrom<usize>) -> &Digits {
        self.index_range(rng.start..self.len())
        // let head = &self[rng.start] as *const Digit;
        // unsafe { Digits::from_raw_parts(head, self.len - rng.start) }
    }

    fn index_range_to(&self, rng: ops::RangeTo<usize>) -> &Digits {
        self.index_range(0..rng.end)
        // unsafe { Digits::from_raw_parts(self.ptr, rng.end) }
    }

    pub unsafe fn quote_mul(&mut self, rng: ops::Range<usize>) -> (&mut Digits, &mut Digits) {
        let part = Digits::from_raw_parts_mut(
            self.ptr.add(rng.start).as_mut().unwrap(),
            rng.end - rng.start,
            self.len as isize - rng.start as isize,
        );
        (part, self)
    }
}

impl ops::Deref for RawDigits {
    type Target = Digits;
    fn deref(&self) -> &Digits {
        self.deref()
    }
}
impl ops::DerefMut for RawDigits {
    fn deref_mut(&mut self) -> &mut Digits {
        self.deref_mut()
    }
}

impl Drop for RawDigits {
    fn drop(&mut self) {
        self.drop()
    }
}

impl ops::Index<usize> for Digits {
    type Output = Digit;
    fn index(&self, idx: usize) -> &Digit {
        self.index(idx)
    }
}
impl ops::IndexMut<usize> for Digits {
    fn index_mut(&mut self, idx: usize) -> &mut Digit {
        self.index_mut(idx)
    }
}

impl ops::Index<ops::Range<usize>> for Digits {
    type Output = Digits;
    fn index(&self, rng: ops::Range<usize>) -> &Digits {
        self.index_range(rng)
    }
}
impl ops::IndexMut<ops::Range<usize>> for Digits {
    fn index_mut(&mut self, rng: ops::Range<usize>) -> &mut Digits {
        self.index_range_mut(rng)
    }
}

impl ops::Index<ops::RangeInclusive<usize>> for Digits {
    type Output = Digits;
    fn index(&self, rng: ops::RangeInclusive<usize>) -> &Digits {
        self.index_range_inclusive(rng)
    }
}

impl ops::Index<ops::RangeFrom<usize>> for Digits {
    type Output = Digits;
    fn index(&self, rng: ops::RangeFrom<usize>) -> &Digits {
        self.index_range_from(rng)
    }
}

impl ops::Index<ops::RangeTo<usize>> for Digits {
    type Output = Digits;
    fn index(&self, rng: ops::RangeTo<usize>) -> &Digits {
        self.index_range_to(rng)
    }
}

impl std::fmt::Debug for Digits {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("[{} : {}]", self.len, self.rlen);
        for i in (0..self.len()).rev() {
            print!(" {:016X}", self[i]);
        }
        return Ok(());
    }
}
