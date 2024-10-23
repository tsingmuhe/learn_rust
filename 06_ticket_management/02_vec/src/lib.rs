pub fn fibonacci(n: u32) -> u32 {
    let n = n as usize;

    let mut v = Vec::with_capacity(n + 1);
    v.push(0);
    v.push(1);

    for i in 2..=n {
        v.push(v[i - 1] + v[i - 2]);
    }

    v[n]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}