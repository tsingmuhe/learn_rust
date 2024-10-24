fn is_even(n: u32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn test_is_even() {
        assert!(!is_even(1));
        assert!(is_even(2));
        assert!(!is_even(231));
    }
}
