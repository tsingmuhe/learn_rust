fn greeting() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(), "Hello, World!");
    }
}