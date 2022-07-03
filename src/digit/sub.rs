use core::panic;

use super::calc::*;
use super::*;

// ans先頭の数字を0で上書きする
pub fn sub(ans: &mut Digits, lhs: &Digits, rhs: &Digits) -> usize {
    let mut diff_borrow = [0, 0];
    let mut ans_len = 0;
    // TODO: iterator(enumerate)にする
    for i in 0..ans.len {
        diff_borrow = sub_borrow([lhs[i], rhs[i], diff_borrow[1]]);
        ans[i] = diff_borrow[0];
        if ans[i] != 0 {
            ans_len = i + 1;
        }
    }
    if diff_borrow[1] > 0 {
        println!("{:?} - {:?}", lhs, rhs);
        panic!("CANNOT CALCULATE (LITTLE - BIG) IN NATURAL NUMBER");
    }
    ans_len
}

pub fn sub_sign(ans: &mut Digits, lhs: &Digits, rhs: &Digits) -> (usize, isize) {
    if less_than(lhs, rhs) {
        let len = sub(ans, rhs, lhs);
        return (len, -1);
    }
    let len = sub(ans, lhs, rhs);
    if len == 0 {
        return (0, 0);
    }
    (len, 1)
    // let mut diff_borrow = [0, 0];
    // let mut ans_len = 0;
    // let mut ans_sign = 0;
    // // TODO: iterator(enumerate)にする
    // for i in 0..ans.len {
    //     diff_borrow = sub_borrow([lhs[i], rhs[i], diff_borrow[1]]);
    //     if diff_borrow[0] > 0 {
    //         ans[i] = diff_borrow[0];
    //         ans_len = 1 + i;
    //         ans_sign = 1;
    //     }
    // }
    // if ans_len == 0 {
    //     return (0, 0);
    // }
    // if diff_borrow[1] > 0 {
    //     let (len, mut sign) = sub_sign(ans, rhs, lhs);
    //     sign *= -1;
    //     return (len, sign);
    // }
    // (ans_len, ans_sign)
}

fn less_than(lhs: &Digits, rhs: &Digits) -> bool {
    for i in (0..std::cmp::max(lhs.len(), rhs.len())).rev() {
        if lhs[i] < rhs[i] {
            return true;
        }
        if lhs[i] > rhs[i] {
            return false;
        }
    }
    false
}

// ans先頭の数字を0で上書きしない
pub fn sub_assign(ans: &mut Digits, rhs: &Digits) -> usize {
    let mut diff_borrow = [0, 0];
    let mut ans_len = 0;
    // TODO: iterator(enumerate)にする
    for i in 0..ans.len {
        diff_borrow = sub_borrow([ans[i], rhs[i], diff_borrow[1]]);
        ans[i] = diff_borrow[0];
        if i >= rhs.len && diff_borrow[1] == 0 {
            ans_len = i + 1;
            break;
        }
    }
    ans_len
}
