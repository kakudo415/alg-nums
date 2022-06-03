pub mod add;
pub mod cmp;
pub mod misc;
pub mod sub;

use std::alloc::alloc;
use std::alloc::Layout;
use std::ptr::copy;
use std::ptr::write_bytes;

pub struct Natural {
    digits: *mut usize,
    length: usize,
    capacity: usize, // TODO: 将来的にはlength==capacityにして一本化？
                     // むずそう。計算前capacity推測を完璧にしないと、capacityが爆発してしまうから。
}

impl Natural {
    fn new(capacity: usize) -> Self {
        let new_layout = Layout::array::<usize>(capacity).unwrap();
        let new_ptr = unsafe { alloc(new_layout) } as *mut usize;
        unsafe {
            write_bytes::<usize>(new_ptr, 0, capacity);
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
