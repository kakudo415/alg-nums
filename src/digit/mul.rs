use super::calc;
use super::*;

use std::cmp;

const KARATSUBA_THRESHOLD: usize = 4;

pub fn mul(ans: &mut Digits, lhs: &Digits, rhs: &Digits, width: usize) -> usize {
    // TODO: FFT
    if width >= KARATSUBA_THRESHOLD {
        return karatsuba(ans, lhs, rhs, width);
    }
    elementary(ans, lhs, rhs)
}

// TODO: ans_lenを賢くする
fn elementary(ans: &mut Digits, lhs: &Digits, rhs: &Digits) -> usize {
    for i in 0..ans.len() {
        ans[i] = 0;
    }
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
            if ans[i + j] != 0 {
                ans_len = cmp::max(ans_len, i + j + 1);
            }
        }
        if internal[1] != 0 {
            ans[i + lhs.len()] = internal[1];
            ans_len = cmp::max(ans_len, i + lhs.len() + 1);
        }
    }

    ans_len
}

// RECURSIONを固定長にするためには、各再帰で"ぴったり半分"にならないとダメ？つまり2ⁿ長？
// x × y = (x0 + x1 × D) × (y0 + y1 × D)
//       = x0 × y0
//         + (x0 × y1 + x1 × y0) × D
//         + (x1 × y1) × D²
//       = x0 × y0
//         - {(x0 - x1) × (y0 - y1) - x0 × y0 - x1 × y1} × D
//         + (x1 × y1) × D²
// Let t0 := x0 × y0, t1 := (x0 - x1) × (y0 - y1), t2 := x1 × y1,
//       = t0 - (t1 - t0 - t2) × m + t2 × m²
// [        x0 × y0        |        x1 × y1        | (x0 - x1) × (y0 - y1) |                               RECURSION                               |  x0 - x1  |  y0 - y1  ]
// [           D           |           D           |           D           |                                 D × 3                                 |   D / 2   |   D / 2   ]
fn karatsuba(ans: &mut Digits, x: &Digits, y: &Digits, d: usize) -> usize {
    if d < KARATSUBA_THRESHOLD {
        return elementary(ans, x, y);
    }

    // for _ in 0..(64 / d) {
    //     print!(" ");
    // }
    // println!("KARATSUBA START {}", d);
    // for _ in 0..(64 / d) {
    //     print!(" ");
    // }
    // println!("{:?}", x);
    // for _ in 0..(64 / d) {
    //     print!(" ");
    // }
    // println!("{:?}", y);

    // 分割
    let x0 = &x[0..d / 2];
    let x1 = &x[d / 2..d];
    let y0 = &y[0..d / 2];
    let y1 = &y[d / 2..d];

    // println!();
    // for _ in 0..(64 / d) {
    //     print!(" ");
    // }
    // println!("x0 = {:?}", x0);
    // for _ in 0..(64 / d) {
    //     print!(" ");
    // }
    // println!("x1 = {:?}", x1);
    // for _ in 0..(64 / d) {
    //     print!(" ");
    // }
    // println!("y0 = {:?}", y0);
    // for _ in 0..(64 / d) {
    //     print!(" ");
    // }
    // println!("y1 = {:?}", y1);

    // t0 = x0 × y0
    let t0_len = karatsuba(&mut ans[d * 0..d * 6], x0, y0, d / 2);
    let (t0, ans) = unsafe { ans.quote_mul(d * 0..d * 0 + t0_len) };
    // println!("t0 =\n{:?}", t0);

    // t2 = x1 × y1
    let t2_len = karatsuba(&mut ans[d * 1..d * 6], x1, y1, d / 2);
    let (t2, ans) = unsafe { ans.quote_mul(d * 1..d * 1 + t2_len) };
    // println!("t2 =\n{:?}", t2);

    // u = x0 - x1
    let (u_len, u_sign) = sub::sub_sign(&mut ans[d * 6..d * 6 + d / 2], x0, x1);
    let (u, ans) = unsafe { ans.quote_mul(d * 6..d * 6 + u_len) };
    // println!("u = {:?}", u);

    // v = y0 - y1
    let (v_len, v_sign) = sub::sub_sign(&mut ans[d * 6 + d / 2..d * 7], y0, y1);
    let (v, ans) = unsafe { ans.quote_mul(d * 6 + d / 2..d * 6 + d / 2 + v_len) };
    // println!("v = {:?}", v);

    match u_sign * v_sign {
        0 => {
            // t1 = u × v = 0
            //   - (t1 - t0 - t2)
            // = + t0 + t2
            let len = add::add(&mut ans[d * 2..d * 4], t0, t2);
            let (t1, ans) = unsafe { ans.quote_mul(d * 2..d * 2 + len) };
            return add::add_assign(&mut ans[d / 2..d * 2], t1) + d / 2;
        }
        1 => {
            // t1 = u × v
            let uv_len = karatsuba(&mut ans[d * 2..d * 6], u, v, d / 2);
            let (t1, ans) = unsafe { ans.quote_mul(d * 2..d * 2 + uv_len) };
            // println!("t1 = {:?}", t1);
            // - (t1 - t0 - t2)
            let (len, sign) = sub::sub_sign(&mut ans[d * 2..d * 4], t1, t0);
            let (t1, ans) = unsafe { ans.quote_mul(d * 2..d * 2 + len) };
            // println!("t1 = {:?}", t1);
            match sign {
                0 => {
                    return add::add_assign(&mut ans[d / 2..d * 2], t2) + d / 2;
                }
                1 => {
                    let len = sub::sub(&mut ans[d * 2..d * 4], t2, t1);
                    return add::add_assign(&mut ans[d / 2..d * 2], &t1[0..len]) + d / 2;
                }
                -1 => {
                    let len = add::add(&mut ans[d * 2..d * 4], t1, t2);
                    let (t1, ans) = unsafe { ans.quote_mul(d * 2..d * 2 + len) };
                    return add::add_assign(&mut ans[d / 2..d * 2], &t1[0..len]) + d / 2;
                }
                _ => {
                    panic!("UNKNOWN ERROR HAS BEEN OCCURED!");
                }
            }
        }
        -1 => {
            // t1 = u × v
            let uv_len = karatsuba(&mut ans[d * 2..d * 6], u, v, d / 2);
            let (t1, ans) = unsafe { ans.quote_mul(d * 2..d * 2 + uv_len) };
            //   - (-t1 - t0 - t2)
            // = + t1 + t0 + t2
            let len = add::add(&mut ans[d * 2..d * 4], t1, t0);
            let len = add::add(&mut ans[d * 2..d * 4], &t1[0..len], t2);
            return add::add_assign(&mut ans[d / 2..d * 2], &t1[0..len]) + d / 2;
        }
        _ => {
            panic!("UNKNOWN ERROR HAS BEEN OCCURED!")
        }
    }
}
