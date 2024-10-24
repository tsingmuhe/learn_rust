fn factorial(n: u32) -> u32 {
    let mut sum = 1;

    for i in 1..=n {
        sum = sum * i;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    #[should_panic]
    fn test_factorial() {
        assert_eq!(factorial(20), 2_192_834_560);
    }
}
