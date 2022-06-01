mod integer;

#[cfg(test)]
mod tests {
    use super::integer::Integer;

    #[test]
    fn test_add_sub() {
        // TODO: 目視・手動検算じゃないテストを書く
        let mut ans = Integer::from(100000000);
        for i in 0..200 {
            println!("{:03}: {:X}", i, ans);
            ans = &ans + &ans;
        }

        let mut ans = Integer::from(-100000000);
        for i in 0..200 {
            println!("{:03}: {:X}", i, ans);
            ans = &ans + &ans;
        }
    }
}
