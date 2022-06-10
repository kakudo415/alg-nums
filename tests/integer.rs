use alg_nums::*;

#[test]
fn test_integer_mul() {
    let i1234567890 = Integer::from(1234567890);
    let im1234567890 = Integer::from(-1234567890);
    let mut ans = Integer::from(1234567890);
    let mut ansm = Integer::from(1234567890);
    for c in 0..256 {
        ans = &ans * &i1234567890;
        ansm = &ansm * &im1234567890;
        if c % 2 == 0 {
            assert!(ans == -&ansm);
        }
    }
}
