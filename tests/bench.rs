use alg_nums::*;

use std::time::*;

#[test]
fn bench_mul() {
    let n = Natural::from(2);
    println!("SCHOOL ALGORITHM: {}ms", school(n.clone(), 24));
}

fn school(mut n: Natural, rep: usize) -> u128 {
    let begin = Instant::now();
    for _ in 0..rep {
        n = &n * &n;
    }
    let end = Instant::now();
    end.duration_since(begin).as_nanos()
}
