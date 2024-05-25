// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread::scope;

pub fn sum(v: Vec<i32,>,) -> i32 {
    let mid = v.len() / 2;
    let (sum1, sum2,) = scope(|scope| {
        let sum1 = scope.spawn(|| -> i32 {
            let first: &[i32] = &v[..mid];
            first.iter().sum()
        },);
        let sum2 = scope.spawn(|| -> i32 {
            let second = &v[mid..];
            second.iter().sum()
        },);
        (sum1.join().unwrap(), sum2.join().unwrap(),)
    },);
    sum1 + sum2
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
