fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    if time_elapsed <= 0 {
        panic!("time_elapsed cannot be negative");
    }

    if start > end {
        panic!("start must not be greater than end");
    }

    let distance = end - start;
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn test_speed() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    #[should_panic]
    fn test_speed_panic_1() {
        speed(0, 10, 0);
    }

    #[test]
    #[should_panic]
    fn test_speed_panic_2() {
        speed(10, 0, 10);
    }
}
