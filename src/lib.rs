mod integer;

#[cfg(test)]
mod tests {
    use super::integer::Integer;

    #[test]
    fn test_add() {
        let zero = Integer::zero();
        let v0 = Integer::from(9);
        let v1 = Integer::from(16);
        let v2 = Integer::from(25);
        println!("{} {} {} {}", zero, v0, v1, v2);
    }
}
