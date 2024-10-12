fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }

    n * factorial(n - 1)
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(5), 120);
    }
}
