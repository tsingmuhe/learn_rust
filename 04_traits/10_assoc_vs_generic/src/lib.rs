trait Power<T> {
    type Output;
    fn power(&self, n: T) -> Self::Output;
}

impl<T> Power<T> for u32 {
    type Output = u32;

    fn power(&self, n: T) -> Self::Output {
        let mut sum: u32 = 1;

        for _ in 1..=n {
            sum = sum * self
        }

        sum
    }
}


#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}