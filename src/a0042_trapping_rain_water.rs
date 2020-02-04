/*
 * [0042] trapping-rain-water
 */

pub struct Solution {}

// solution impl starts here

use std::cmp::max;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let (mut left, mut right, mut depth, mut result) = (0, height.len() - 1, std::i32::MIN, 0);
        while left < right {
            let p;
            if height[left] < height[right] {
                p = left;
                left += 1;
            } else {
                p = right;
                right -= 1;
            }
            depth = max(depth, height[p]);
            if depth > height[p] {
                result += depth - height[p];
            }
        }
        result
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}

// solution tests ends here
