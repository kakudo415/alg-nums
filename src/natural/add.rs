use super::Natural;

use std::cmp;
use std::ops::Add;

const MSB_MASK: usize = 1 << (usize::BITS - 1); // Most Significant Bit

impl Add for &Natural {
    type Output = Natural;

    fn add(self, rhs: Self) -> Natural {
        let mut answer = Natural::new(needed_capacity(self, rhs));

        let mut digit = (0, 0); // (sum, carried)
        for i in 0..answer.capacity {
            digit = add_carry(self[i], rhs[i], digit.1);
            answer[i] = digit.0;
        }
        answer.fit();
        answer
    }
}

#[inline]
fn add_carry(lhs: usize, rhs: usize, carry: usize) -> (usize, usize) {
    let sum = (lhs & !MSB_MASK) + (rhs & !MSB_MASK) + carry;
    match msb(sum) + msb(lhs) + msb(rhs) {
        0 => (sum, 0),
        1 => (sum + 1, 0),
        2 => (sum, 1),
        3 => (sum + 1, 1),
        _ => {
            panic!("add_carry() made something wrong...");
        }
    }
}

#[inline]
fn msb(u: usize) -> usize {
    u >> (usize::BITS - 1)
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length) + 1 // TODO: もうちょっと賢くする
}
