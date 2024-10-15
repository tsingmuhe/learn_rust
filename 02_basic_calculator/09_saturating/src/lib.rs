fn factorial(n: u32) -> u32 {
    let mut sum: u32 = 1;

    for i in 1..=n {
        sum = sum.saturating_mul(i);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(20), u32::MAX);
    }
}