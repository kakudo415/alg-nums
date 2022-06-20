use super::add::*;
use super::digit::*;
use super::sub::*;
use super::Natural;

use std::ops::Mul;

// TODO: ちゃんと計測して閾値を決める
const KARATSUBA_THRESHOLD: usize = 4;
const FFT_THRESHOLD: usize = 256;

impl Mul for &Natural {
    type Output = Natural;

    fn mul(self, other: Self) -> Natural {
        let (lhs, rhs) = if self.length > other.length {
            (self, other)
        } else {
            (other, self)
        };

        // if rhs.length >= FFT_THRESHOLD {
        //     return fft(lhs, rhs);
        // }
        karatsuba(lhs, rhs)
    }
}

fn _grade_school(answer: &mut [Digit], lhs: &[Digit], rhs: &[Digit]) {
    for i in 0..rhs.len() {
        if rhs[i] == 0 {
            continue;
        }
        let mut internal_answer = [0, 0];
        for j in 0..lhs.len() {
            internal_answer = mul_carry(rhs[i], lhs[j], internal_answer[1], answer[i + j]);
            answer[i + j] = internal_answer[0];
        }
        if internal_answer[1] != 0 {
            answer[i + lhs.len()] = internal_answer[1];
        }
    }
}

fn karatsuba(lhs: &Natural, rhs: &Natural) -> Natural {
    let mut answer = Natural::new(needed_capacity(lhs, rhs));
    _karatsuba(&mut answer[0..], &lhs[0..], &rhs[0..]);
    answer.normalize();
    answer
}

// 暫定版, TODO: メモリ割り当て無し版を作る
fn _karatsuba(answer: &mut [Digit], lhs: &[Digit], rhs: &[Digit]) {
    if rhs.len() <= KARATSUBA_THRESHOLD {
        return _grade_school(answer, lhs, rhs);
    }

    let unit = lhs.len() + rhs.len();
    let mid = rhs.len() / 2;
    let lhs = lhs.split_at(mid);
    let rhs = rhs.split_at(mid);

    _karatsuba(&mut answer[..mid * 2], lhs.0, rhs.0);
    _karatsuba(&mut answer[mid * 2..], lhs.1, rhs.1);

    let mut buffer = Natural::new(unit * 4);
    let buffer = (&mut buffer[0..]).split_at_mut(unit * 2);
    let ll = _add(&mut buffer.1[0..unit], lhs.0, lhs.1);
    let rl = _add(&mut buffer.1[unit..unit * 2], rhs.0, rhs.1);
    _karatsuba(buffer.0, &buffer.1[0..ll], &buffer.1[unit..unit + rl]);

    _sub_assign(buffer.0, &answer[..mid * 2]);
    _sub_assign(buffer.0, &answer[mid * 2..]);

    _add_assign(&mut answer[mid..], buffer.0);
}

//   (x0 + x1 × B) × (y0 + y1 × B)
// = (x0 × y0)
//   + (x0 × y1 + x1 × y0) × B
//   + (x1 × y1) × B^2
// = (x0 × y0)
//   + {(x0 + x1) × (y0 + y1) - (x0 × y0) - (x1 × y1)} × B
//   + (x1 × y1) × B^2
// = t0
//   + {t01t01 - t00 - t11} × B
//   + t2 × B^2
// = t0
//   + t1 × B
//   + t2 × B^2

// [x0 × y0 | x1 × y1 |        t2         |                                                 | x0 + x1 | y0 + y1 ]
//     B         B              B                                B × 5                           B         B
// fn _karatsuba(answer: &mut [Digit], lhs: &[Digit], rhs: &[Digit]) {
//     if rhs.len() == 1 {
//         _grade_school(answer, lhs, rhs);
//         return;
//     }

//     let unit = lhs.len() + rhs.len();

//     let mid = rhs.len() / 2;
//     let lhs = lhs.split_at(mid);
//     let rhs = rhs.split_at(mid);

//     _add(&mut answer[(unit * 5)..(unit * 5 + mid + 1)], lhs.0, rhs.0);
//     _add(&mut answer[(unit * 5 + mid + 1)..(unit * 6)], lhs.1, rhs.1);

//     // 他の項のことは考えずにメモリを使ってよい。結果だけは上書きしないように、answerの範囲の前半に気を付ける。
//     _karatsuba(&mut answer[0..unit * 3], lhs.0, rhs.0);

//     _karatsuba(&mut answer[unit..unit * 4], lhs.1, rhs.1);

//     _karatsuba(
//         &mut answer[unit * 2..unit * 5],
//         &answer[unit * 5..unit * 5 + mid + 1],
//         &answer[unit * 5 + mid + 1..unit * 6],
//     );

//     _sub_assign(&mut answer[unit * 0..unit * 2], &answer[unit * 0..unit * 1]);
//     _sub_assign(&mut answer[unit * 0..unit * 2], &answer[unit * 1..unit * 2]);

//     _add_assign(&mut answer[mid..unit * 2], &answer[unit * 2..unit * 3]);
// }

fn fft(lhs: &Natural, rhs: &Natural) -> Natural {
    todo!();
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    lhs.length + rhs.length
}
