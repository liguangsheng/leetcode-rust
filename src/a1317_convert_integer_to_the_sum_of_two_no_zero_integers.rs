/*
 * [1317] convert-integer-to-the-sum-of-two-no-zero-integers
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 0..n {
            if !Solution::contains_zero(i) && !Solution::contains_zero(n - i) {
                return vec![i, n - i];
            }
        }
        vec![]
    }

    fn contains_zero(mut n: i32) -> bool {
        if n == 0 {
            return true;
        }
        while n > 0 {
            if n % 10 == 0 {
                return true;
            }
            n /= 10;
        }
        false
    }
}
// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
        assert_eq!(Solution::get_no_zero_integers(10000), vec![1, 9999]);
        assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
    }
}

// solution tests ends here
