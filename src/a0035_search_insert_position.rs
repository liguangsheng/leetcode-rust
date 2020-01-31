/*
 * [0035] search-insert-position
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = nums.len() as i32 - 1;
        let mut mid;

        while low < high {
            mid = (low + high) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        if target <= nums[low as usize] {
            return low as i32;
        }
        low + 1
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test_case1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::search_insert(vec![1, 3], 0), 0);
    }

    #[test]
    fn test_case5() {
        assert_eq!(Solution::search_insert(vec![1, 3], 1), 0);
    }

    #[test]
    fn test_case6() {
        assert_eq!(Solution::search_insert(vec![1], 1), 0);
    }

    #[test]
    fn test_case7() {
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);
    }
}

// solution tests ends here
