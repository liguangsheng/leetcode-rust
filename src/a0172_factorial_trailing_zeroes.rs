/*
 * [0172] factorial-trailing-zeroes
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n > 0 {
            count += n / 5;
            n /= 5;
        }
        count
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
    }
}

// solution tests ends here
