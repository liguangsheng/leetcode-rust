/*
 * [1287] element-appearing-more-than-25-in-sorted-array
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let t = arr.len() / 4;
        for i in 0..arr.len() - 1 {
            if arr[i] == arr[i + t] {
                return arr[i];
            }
        }
        arr[0]
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
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 1, 2, 2, 3, 3, 3, 3]),
            3
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::find_special_integer(vec![1, 2, 3, 3]), 3);
    }
}

// solution tests ends here
