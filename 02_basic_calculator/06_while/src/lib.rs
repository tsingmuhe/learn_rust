fn factorial(n: u32) -> u32 {
    let mut sum: u32 = 1;
    let mut i: u32 = 1;

    while i <= n {
        sum = sum * i;
        i = i + 1;
    }

    sum
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