fn factorial(n: usize) -> usize {
    let mut sum = 1;

    for i in 1..=n {
        sum *= i;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(120, factorial(5));
    }
}
