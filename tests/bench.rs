use alg_nums::*;

use std::time::*;

#[test]
fn bench_mul() {
    let n = Natural::from(2);
    println!("MUL BENCHMARK: {}ms", mul(n.clone(), 12));
}

fn mul(mut n: Natural, rep: usize) -> u128 {
    let begin = Instant::now();
    for _ in 0..rep {
        print!("{:X}² = ", n);
        n = &n * &n;
        println!("{:X}", n);
    }
    println!("{:X}", n);
    let end = Instant::now();
    end.duration_since(begin).as_nanos()
}
