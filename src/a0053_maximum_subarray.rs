/*
 * [0053] maximum-subarray
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = nums[0];

        for i in nums {
            if sum > 0 {
                sum += i;
            } else {
                sum = i;
            }
            res = std::cmp::max(res, sum);
        }
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
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}

// solution tests ends here
