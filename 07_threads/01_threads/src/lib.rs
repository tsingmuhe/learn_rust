use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);
    let v1 = v1.to_vec();
    let v2 = v2.to_vec();

    let h1 = thread::spawn(move || v1.iter().sum::<i32>());
    let h2 = thread::spawn(move || v2.iter().sum::<i32>());

    let a = h1.join().unwrap();
    let b = h2.join().unwrap();

    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}