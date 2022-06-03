mod natural;
mod integer;

#[cfg(test)]
mod tests {
    use super::integer::Integer;

    #[test]
    fn test_add() {
        let v0 = Integer::zero();
        let v9 = Integer::from(9);
        let v16 = Integer::from(16);
        let v25 = Integer::from(25);
        assert!(&v9 + &v16 == v25);
        assert!(&v0 + &v25 == v25);

        let mut ans9 = Integer::zero();
        let mut ans16 = Integer::zero();
        for _ in 0..256 {
            for _ in 0..16 {
                ans9 = &ans9 + &v9;
            }
            for _ in 0..9 {
                ans16 = &ans16 + &v16;
            }
            assert!(ans9 == ans16);
        }
    }
}
