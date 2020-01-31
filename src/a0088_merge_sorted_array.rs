/*
 * [0088] merge-sorted-array
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);
        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        while i >= 0 {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_case1() {
        let mut nums1 = vec![1, 2, 3, 4, 0, 0, 0];
        Solution::merge(&mut nums1, 4, &mut vec![2, 5, 6], 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 4, 5, 6]);
    }
}

// solution tests ends here
