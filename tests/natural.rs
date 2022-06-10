use alg_nums::rational::*;

#[test]
fn add() {
    let n0 = Integer::zero();
    let n9 = Integer::from(9);
    let n16 = Integer::from(16);
    let n25 = Integer::from(25);
    assert!(&n9 + &n16 == n25);
    assert!(&n0 + &n25 == n25);

    let mut ans9 = Integer::zero();
    let mut ans16 = Integer::zero();
    for _ in 0..256 {
        for _ in 0..16 {
            ans9 = &ans9 + &n9;
        }
        for _ in 0..9 {
            ans16 = &ans16 + &n16;
        }
        assert!(ans9 == ans16);
    }
}

#[test]
fn mul() {
    let n6 = Natural::from(6);
    let n216 = Natural::from(216);
    let mut ans6 = Natural::from(1);
    let mut ans216 = Natural::from(1);
    for _ in 0..256 {
        ans6 = &ans6 * &n6;
        ans6 = &ans6 * &n6;
        ans6 = &ans6 * &n6;
        ans216 = &ans216 * &n216;
        assert!(ans6 == ans216);
    }
}

#[test]
fn add_mul() {
    let n2 = Natural::from(2);
    let mut ans_add = Natural::from(1234567890);
    let mut ans_mul = Natural::from(1234567890);
    for _ in 0..256 {
        ans_add = &ans_add + &ans_add;
        ans_mul = &ans_mul * &n2;
        assert!(ans_add == ans_mul);
    }
}
