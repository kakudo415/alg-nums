mod integer;

#[cfg(test)]
mod tests {
    use super::integer::Integer;

    #[test]
    fn test_add() {
        let v0 = Integer::from(usize::MAX);
        let v1 = Integer::from(usize::MAX);
        println!("{} + {} = {}", v0.debug_message(), v1.debug_message(), (&v0 + &v1).debug_message());
    }
}
