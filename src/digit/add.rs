use super::calc::*;
use super::*;

pub fn add(ans: &mut DigitsSlice, lhs: &DigitsSlice, rhs: &DigitsSlice) -> usize {
    let mut sum_carried = [0, 0];
    let mut ans_len = 0;
    // TODO: iterator(enumerate)にする
    for i in 0..ans.len {
        sum_carried = add_carry([lhs[i], rhs[i], sum_carried[1]]);
        ans[i] = sum_carried[0];
        ans_len += 1;
        if i >= lhs.len && i >= rhs.len && sum_carried[1] == 0 {
            break;
        }
    }
    ans_len
}
