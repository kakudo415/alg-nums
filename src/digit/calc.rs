use super::*;

const HALF_WIDTH: u32 = Digit::BITS / 2;
const LOW_BITS: Digit = Digit::MAX >> HALF_WIDTH;

// N <= 2^(Digit::BITS / 2)
pub(crate) fn add_carry<const N: usize>(operands: [Digit; N]) -> [Digit; 2] {
    let mut buffer = [0, 0];

    for operand in &operands {
        buffer[0] += operand & LOW_BITS;
        buffer[1] += operand >> HALF_WIDTH;
    }
    buffer[1] += buffer[0] >> HALF_WIDTH;

    buffer[0] = (buffer[0] & LOW_BITS) | (buffer[1] << HALF_WIDTH);
    buffer[1] = buffer[1] >> HALF_WIDTH;

    buffer
}

// 0 < N
pub(crate) fn sub_borrow<const N: usize>(operands: [Digit; N]) -> [Digit; 2] {
    let mut buffer = [operands[0], 0];

    for i in 1..N {
        let operand = operands[i];
        if buffer[0] >= operand {
            buffer[0] -= operand;
        } else {
            buffer[0] = Digit::MAX - operand + buffer[0] + 1;
            buffer[1] += 1;
        }
    }

    buffer
}

pub(crate) fn mul_carry(lhs: Digit, rhs: Digit, carry: Digit, inter: Digit) -> [Digit; 2] {
    let lhs_lo = lhs & LOW_BITS;
    let lhs_hi = lhs >> HALF_WIDTH;
    let rhs_lo = rhs & LOW_BITS;
    let rhs_hi = rhs >> HALF_WIDTH;

    let lo = lhs_lo * rhs_lo; // <= 11100001
    let mid_0 = lhs_lo * rhs_hi; // <= 11100001
    let mid_1 = lhs_hi * rhs_lo; // <= 11100001
    let hi = lhs_hi * rhs_hi; // <= 11100001

    let lo = add_carry([lo, mid_0 << HALF_WIDTH, mid_1 << HALF_WIDTH, carry, inter]); // <= 00000010 11111111
    let hi = hi + (mid_0 >> HALF_WIDTH) + (mid_1 >> HALF_WIDTH) + lo[1]; // <= 11111111

    [lo[0], hi]
}
