use super::*;

use std::fmt;
use std::ops;

impl ops::Index<usize> for Integer {
    type Output = usize;
    fn index(&self, idx: usize) -> &usize {
        &self.abs_value[idx]
    }
}

impl ops::IndexMut<usize> for Integer {
    fn index_mut(&mut self, idx: usize) -> &mut usize {
        &mut self.abs_value[idx]
    }
}

impl From<isize> for Integer {
    fn from(value: isize) -> Self {
        if value == 0 {
            return Integer::zero();
        }
        let mut new_integer = Integer::new(1);
        new_integer.sign = if value > 0 { Sign::Plus } else { Sign::Minus };
        new_integer[0] = value.abs() as usize;
        new_integer
    }
}

impl fmt::UpperHex for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.sign == Sign::Minus { "-" } else { " " };
        write!(f, "{}{:X}", sign, self.abs_value)
    }
}
