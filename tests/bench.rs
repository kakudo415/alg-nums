use alg_nums::*;

use std::time::*;

#[test]
fn bench_mul() {
    let n = Natural::from(111);
    println!("MUL BENCHMARK: {}ms", mul(n.clone(), 8));
}

fn mul(mut n: Natural, rep: usize) -> u128 {
    let begin = Instant::now();
    for _ in 0..rep {
        n = &n * &n;
    }
    println!("{:X}", n);
    let end = Instant::now();
    end.duration_since(begin).as_nanos()
}
