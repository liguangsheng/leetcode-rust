/*
 * [0066] plus-one
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            digits[i] += 1;
            digits[i] %= 10;
            if digits[i] != 0 {
                return digits;
            }
        }
        let mut res = vec![1];
        res.append(&mut digits);
        res
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_case1() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}

// solution tests ends here
