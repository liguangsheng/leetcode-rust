/*
 * [0004] median-of-two-sorted-arrays
 */

pub struct Solution {}

// solution impl starts here

use std::cmp::{max, min};
use std::i32::{MAX, MIN};
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());

        if m > n {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let (mut low, mut high) = (0, 2 * m);
        let (mut lmax1, mut lmax2, mut rmin1, mut rmin2) = (0, 0, 0, 0);
        while low <= high {
            let c1 = (low + high) / 2;
            let c2 = m + n - c1;

            lmax1 = if c1 == 0 { MIN } else { nums1[(c1 - 1) / 2] };
            lmax2 = if c2 == 0 { MIN } else { nums2[(c2 - 1) / 2] };
            rmin1 = if c1 == m * 2 { MAX } else { nums1[c1 / 2] };
            rmin2 = if c2 == n * 2 { MAX } else { nums2[c2 / 2] };

            if lmax1 > rmin2 {
                high = c1 - 1;
            } else if lmax2 > rmin1 {
                low = c1 + 1;
            } else {
                break;
            }
        }
        (max(lmax1, lmax2) + min(rmin1, rmin2)) as f64 / 2.0
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
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2f64
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5f64
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1f64);
    }
}

// solution tests ends here
