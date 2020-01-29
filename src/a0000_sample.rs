#![allow(dead_code)]
struct Solution {}

impl Solution {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(5, Solution::add(2, 3));
    }
}
