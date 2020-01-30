/*
 * [1200] minimum-absolute-difference
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        let (mut first, mut second) = (0usize, 1usize);
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut min_diff = std::i32::MAX;
        while second < arr.len() {
            let (a, b) = (arr[first], arr[second]);
            let diff = b - a;
            if diff < min_diff {
                min_diff = diff;
                res.clear();
            }
            if diff == min_diff {
                res.push(vec![a, b]);
            }
            first += 1;
            second += 1;
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
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
            vec![[1, 2], [2, 3], [3, 4]]
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
            vec![[1, 3]]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            vec![[-14, -10], [19, 23], [23, 27]]
        );
    }
}

// solution tests ends here
