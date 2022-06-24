use super::calc::*;
use super::*;

pub fn sub(ans: &mut DigitsSlice, lhs: &DigitsSlice, rhs: &DigitsSlice) -> usize {
    let mut diff_borrow = [0, 0];
    let mut ans_len = 0;
    // TODO: iterator(enumerate)にする
    for i in 0..ans.len {
        diff_borrow = sub_borrow([lhs[i], rhs[i], diff_borrow[1]]);
        ans[i] = diff_borrow[0];
        ans_len += 1;
        if i >= lhs.len && i >= rhs.len && diff_borrow[1] == 0 {
            break;
        }
    }
    ans_len
}


// pub(crate) fn _sub(answer: &mut [Digit], lhs: &[Digit], rhs: &[Digit]) {
//     let mut digit = [0, 0]; // (diff, borrow)
//     for i in 0..answer.len() {
//         let ld = if i < lhs.len() { lhs[i] } else { 0 };
//         let rd = if i < rhs.len() { rhs[i] } else { 0 };
//         digit = sub_borrow([ld, rd, digit[1]]);
//         answer[i] = digit[0];
//     }
//     if digit[1] > 0 {
//         panic!("Cannot calculate (LITTLE - BIG) in Natural Number");
//     }
// }
