use super::Natural;

use std::cmp;
use std::ops::Add;

const MSB: usize = 1 << (usize::BITS - 1); // Most Significant Bit

impl Add for &Natural {
    type Output = Natural;

    fn add(self, rhs: Self) -> Natural {
        let mut sum = Natural::new(needed_capacity(self, rhs));
        sum.length = cmp::max(self.length, rhs.length);

        let mut carry = 0;
        for i in 0..sum.length {
            let lhd = self[i] & (!MSB);
            let rhd = rhs[i] & (!MSB);
            sum[i] = lhd + rhd + carry;
            // 繰り上がり処理あってる？
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
        sum.fit();
        sum
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length) + 1 // TODO: もうちょっと賢くする
}
