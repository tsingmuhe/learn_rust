fn intro() -> &'static str {
    "hello world!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "hello world!");
    }
}