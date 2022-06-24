use super::calc;
use super::*;

use std::cmp;

pub fn mul(ans: &mut DigitsSlice, lhs: &DigitsSlice, rhs: &DigitsSlice) -> usize {
    // TODO: FFT
    karatsuba(ans, lhs, rhs)
}

fn elementary(ans: &mut DigitsSlice, lhs: &DigitsSlice, rhs: &DigitsSlice) -> usize {
    let mut ans_len = 0;

    // TODO: iteratorにする
    for i in 0..rhs.len() {
        if rhs[i] == 0 {
            continue;
        }

        let mut internal = [0, 0];
        for j in 0..lhs.len() {
            internal = calc::mul_carry(rhs[i], lhs[j], internal[1], ans[i + j]);
            ans[i + j] = internal[0];
        }
        if internal[1] != 0 {
            ans[i + lhs.len()] = internal[1];
            ans_len = cmp::max(ans_len, i + lhs.len() + 1);
        } else {
            ans_len = cmp::max(ans_len, i + lhs.len());
        }
    }

    ans_len
}

fn karatsuba(ans: &mut DigitsSlice, lhs: &DigitsSlice, rhs: &DigitsSlice) -> usize {
    if lhs.len() < rhs.len() {
        return karatsuba(ans, rhs, lhs);
    }
    elementary(ans, lhs, rhs)
    // TODO: Karatsuba
}
