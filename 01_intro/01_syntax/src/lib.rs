fn compute(a: i32, b: i32) -> i32 {
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn test_compute() {
        assert_eq!(compute(1, 2), 5);
    }
}
