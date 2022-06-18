use super::add_carry;
use super::Digits;

pub(crate) fn partial_add(
    answer: &mut Digits,
    ahead: usize,
    lhs: &Digits,
    (lhead, llen): (usize, usize),
    rhs: &Digits,
    (rhead, rlen): (usize, usize),
) {
    let mut sum_carried = [0, 0]; // (sum, carried)
    for i in 0..answer.capacity {
        sum_carried = add_carry([lhs[i], rhs[i], sum_carried[1]]);
        answer[i] = sum_carried[0];
    }
}
