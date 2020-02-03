/*
 * [0646] maximum-length-of-pair-chain
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(5, Solution::add(2, 3));
    }
}

// solution tests ends here
