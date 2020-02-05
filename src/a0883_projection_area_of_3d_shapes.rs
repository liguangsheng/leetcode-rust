/*
 * [0883] projection-area-of-3d-shapes
 */

pub struct Solution {}

// solution impl starts here

use std::cmp::max;
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        let (mut max_row, mut max_col);
        for i in 0..grid.len() {
            max_row = 0;
            max_col = 0;
            for j in 0..grid.len() {
                area += if grid[i][j] > 0 { 1 } else { 0 };
                max_row = max(max_row, grid[i][j]);
                max_col = max(max_col, grid[j][i]);
            }
            area += max_row + max_col;
        }
        area
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    }
}

// solution tests ends here
