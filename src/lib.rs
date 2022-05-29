mod integer;

#[cfg(test)]
mod tests {
    use super::integer::Integer;

    #[test]
    fn test_add() {
        let mut sum = Integer::from(100000000);
        for i in 0..200 {
            sum = &sum + &sum;
            println!("{:03}: {:X}", i, sum);
        }
    }
}
