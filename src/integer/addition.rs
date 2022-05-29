use super::Integer;

use std::cmp;
use std::ops::Add;

const MSB: usize = 1 << (usize::BITS - 1); // Most Significant Bit

impl Add for &Integer {
    type Output = Integer;
    // TODO: 負の整数に対応
    fn add(self, rhs: Self) -> Integer {
        let mut sum = Integer::new(needed_memory_cap(self, rhs));
        sum.length = cmp::max(self.length, rhs.length);
        let mut carry = 0;
        for i in 0..sum.length {
            let lhd = self[i] & (!MSB);
            let rhd = rhs[i] & (!MSB);
            sum[i] = lhd + rhd + carry;
            carry = if (self[i] & MSB > 0) && (rhs[i] & MSB > 0) {
                1
            } else {
                0
            };
        }
        if carry > 0 {
            let length = sum.length;
            sum[length] = carry;
        }
        sum
    }
}

fn needed_memory_cap(lhs: &Integer, rhs: &Integer) -> usize {
    cmp::max(lhs.length, rhs.length) + 1 // TODO: もうちょっと賢くする
}
