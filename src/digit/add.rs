use super::calc::*;
use super::*;

// ans先頭の数字を0で上書きする
pub fn add(ans: &mut Digits, lhs: &Digits, rhs: &Digits) -> usize {
    let mut sum_carried = [0, 0];
    let mut ans_len = 0;
    // TODO: iterator(enumerate)にする
    for i in 0..ans.len {
        sum_carried = add_carry([lhs[i], rhs[i], sum_carried[1]]);
        ans[i] = sum_carried[0];
        if i < lhs.len || i < rhs.len || sum_carried[1] != 0 {
            ans_len += 1;
        }
    }
    ans_len
}

// ans先頭の数字を0で上書きしない
pub fn add_assign(ans: &mut Digits, rhs: &Digits) -> usize {
    let mut sum_carried = [0, 0];
    let mut ans_len = 0;
    // TODO: iterator(enumerate)にする
    for i in 0..ans.len {
        sum_carried = add_carry([ans[i], rhs[i], sum_carried[1]]);
        ans[i] = sum_carried[0];
        if ans[i] != 0 {
            ans_len = i + 1;
        }
    }
    ans_len
}
