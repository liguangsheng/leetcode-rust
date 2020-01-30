/*
 * [0931] minimum-falling-path-sum
 */

pub struct Solution {}

// solution impl starts here

use std::cmp::min;
impl Solution {
    pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
        let size = a.len();
        let mut dp = a;
        for row in 2..=size {
            let row = size - row;
            for col in 0..size {
                let next_row = row + 1;
                match col {
                    col if col == 0 => {
                        dp[row][col] += min(dp[next_row][col], dp[next_row][col + 1])
                    }
                    col if col == size - 1 => {
                        dp[row][col] += min(dp[next_row][col], dp[next_row][col - 1])
                    }
                    col => {
                        dp[row][col] += Solution::min3(
                            dp[next_row][col - 1],
                            dp[next_row][col],
                            dp[next_row][col + 1],
                        )
                    }
                }
            }
        }

        let mut res = std::i32::MAX;
        for i in &dp[0] {
            res = min(res, *i);
        }
        res
    }

    #[inline]
    pub fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T {
        v3.min(v1.min(v2))
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
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            12
        );
    }
}

// solution tests ends here
