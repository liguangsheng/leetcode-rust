/*
 * [0026] remove-duplicates-from-sorted-array
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        while i < nums.len() {
            if nums[i - 1] == nums[i] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        nums.len() as i32
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn test_case1() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}

// solution tests ends here
