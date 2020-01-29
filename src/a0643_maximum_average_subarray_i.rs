/*
 * [0643] maximum-average-subarray-i
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max_sum;
        let mut sum = 0;
        for i in 0..=k - 1 {
            sum += nums[i as usize];
        }
        max_sum = sum;
        for i in k..=(nums.len() - 1) as i32 {
            sum = sum - nums[(i - k) as usize] + nums[i as usize];
            max_sum = if sum > max_sum { sum } else { max_sum };
        }

        max_sum as f64 / k as f64
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::find_max_average(vec![1, 2, 3, 4], 2), 3.5f64);
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75f64
        );
    }
}

// solution tests ends here
