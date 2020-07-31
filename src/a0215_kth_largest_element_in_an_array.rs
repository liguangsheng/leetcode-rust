/*
 * [0215] kth-largest-element-in-an-array
 */

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let end = nums.len() - 1;
        Self::quick_select(&mut nums, 0usize, end, k as usize)
    }

    fn quick_select(nums: &mut Vec<i32>, start: usize, end: usize, k: usize) -> i32 {
        let p = nums[start];
        let mut left = start + 1;
        let mut right = end;

        while left <= right {
            while left <= right && nums[left] >= p {
                left += 1;
            }
            while left <= right && nums[right] <= p {
                right -= 1;
            }
            if left <= right && nums[left] < p && nums[right] > p {
                nums.swap(left, right);
            }
        }
        nums.swap(start, right);
        if right - start == k - 1 {
            let mut min = nums[0];
            for i in start..start + k as usize {
                if nums[i] < min {
                    min = nums[i];
                }
            }
            return min;
        } else if right - start < k - 1 {
            return Self::quick_select(nums, right + 1, end, k + start - right - 1);
        } else if right - start > k - 1 {
            return Self::quick_select(nums, start, right - 1, k);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::find_kth_largest(
                vec![7, 3, 82, 9, 2, 17, 4, 29, 1, 8, 36, 33, 18, 22, 19],
                2
            ),
            36
        );
        assert_eq!(Solution::find_kth_largest(vec![7, 6, 5, 4, 3, 2, 1], 5), 3);
        assert_eq!(Solution::find_kth_largest(vec![5, 2, 4, 1, 3, 6, 0], 4), 3)
    }
}
