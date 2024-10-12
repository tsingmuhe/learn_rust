use std::ops::Add;

fn greeting() -> &'static str {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
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
