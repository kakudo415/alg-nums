use super::Integer;
use super::Sign;

use std::cmp;
use std::ops::Add;

const MSB: usize = 1 << (usize::BITS - 1); // Most Significant Bit

impl Add for &Integer {
    type Output = Integer;

    fn add(self, rhs: Self) -> Integer {
        match (self.sign, rhs.sign) {
            (_, Sign::Undefined) => self.clone(),
            (Sign::Undefined, _) => rhs.clone(),
            (Sign::Plus, Sign::Plus) | (Sign::Minus, Sign::Minus) => add_same_signs(self, rhs),
            (Sign::Plus, Sign::Minus) => add_plus_minus(self, rhs),
            (Sign::Minus, Sign::Plus) => add_plus_minus(rhs, self),
        }
    }
}

fn add_same_signs(lhs: &Integer, rhs: &Integer) -> Integer {
    let mut sum = Integer::new(needed_same_sign_capacity(lhs, rhs));
    sum.length = cmp::max(lhs.length, rhs.length);
    let mut carry = 0;
    for i in 0..sum.length {
        let lhd = lhs[i] & (!MSB);
        let rhd = rhs[i] & (!MSB);
        sum[i] = lhd + rhd + carry;
        carry = if (lhs[i] & MSB > 0) && (rhs[i] & MSB > 0) {
            1
        } else {
            0
        };
    }
    if carry > 0 {
        let length = sum.length;
        sum[length] = carry;
    }
    sum.sign = lhs.sign;
    sum
}

fn add_plus_minus(lhs: &Integer, rhs: &Integer) -> Integer {
    let mut sum = Integer::new(needed_plus_minus_capacity(lhs, rhs));

    let arhs = &rhs.abs();
    if lhs < arhs {
        return add_plus_minus(arhs, &(-lhs));
    }

    let mut lhs = lhs.clone(); // TODO: 左辺をclone()しなくても繰り下がり処理をできるようにする
    for i in 0..sum.length {
        if lhs[i] >= rhs[i] {
            sum[i] = lhs[i] - rhs[i];
        } else {
            sum[i] = (usize::MAX - (rhs[i] - lhs[i])) + 1; // 繰り下げて計算
            for j in (i + 1)..sum.length {
                if lhs[j] > 0 {
                    lhs[j] -= 1;
                    break;
                }
                lhs[j] = usize::MAX;
            }
        }
    }
    sum
}

fn needed_same_sign_capacity(lhs: &Integer, rhs: &Integer) -> usize {
    cmp::max(lhs.length, rhs.length) + 1 // TODO: もうちょっと賢くする
}

fn needed_plus_minus_capacity(lhs: &Integer, rhs: &Integer) -> usize {
    cmp::max(lhs.length, rhs.length) // TODO: もうちょっと賢くする
}
