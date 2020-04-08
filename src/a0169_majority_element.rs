/*
 * [0169] majority-element
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut count = 0;
        for j in nums {
            if count == 0 {
                i = j;
            }
            count += if j == i { 1 } else { -1 }
        }
        i
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![3, 3, 4]), 3);
    }

    #[test]
    fn test_case1() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}

// solution tests ends here
