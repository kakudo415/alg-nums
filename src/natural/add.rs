use super::super::digit;
use super::Natural;

use std::cmp;
use std::ops::Add;

impl Add for &Natural {
    type Output = Natural;

    fn add(self, other: Self) -> Natural {
        let mut answer = digit::RawDigits::with_capacity(needed_capacity(self, other));
        digit::add::add(&mut answer, &self, &other);
        Natural::from(answer.deref()) // FIXME: なんで暗黙的にDerefされないの？
    }
}

// pub(crate) fn _add_assign(answer: &mut [Digit], rhs: &[Digit]) -> usize {
//     let mut sum_carried = [0, 0]; // (sum, carried)
//     let mut answer_length = 0;
//     for i in 0..answer.len() {
//         let rd = if i < rhs.len() { rhs[i] } else { 0 };
//         sum_carried = add_carry([answer[i], rd, sum_carried[1]]);
//         answer[i] = sum_carried[0];
//         answer_length += 1;
//         if i >= rhs.len() && sum_carried[1] == 0 {
//             break;
//         }
//     }
//     answer_length
// }

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.len, rhs.len) + 1
}
