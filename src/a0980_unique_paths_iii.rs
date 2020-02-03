/*
 * [0980] unique-paths-iii
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (mut x, mut r, mut c) = (1, 0, 0);
        for ir in 0..grid.len() {
            let row = &grid[ir];
            for ic in 0..row.len() {
                if row[ic] == 0 {
                    x += 1;
                } else if row[ic] == 1 {
                    r = ir as i32;
                    c = ic as i32;
                }
            }
        }

        Solution::dfs(&mut grid, r, c, x)
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, x: i32) -> i32 {
        if r < 0
            || c < 0
            || r >= grid.len() as i32
            || c >= grid[0].len() as i32
            || grid[r as usize][c as usize] == -1
        {
            return 0;
        }

        let (ur, uc) = (r as usize, c as usize);
        if grid[ur][uc] == 2 {
            return if x == 0 { 1 } else { 0 };
        }

        grid[ur][uc] = -1;
        let res = Solution::dfs(grid, r + 1, c, x - 1)
            + Solution::dfs(grid, r - 1, c, x - 1)
            + Solution::dfs(grid, r, c + 1, x - 1)
            + Solution::dfs(grid, r, c - 1, x - 1);
        grid[ur][uc] = 0;
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
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
            2
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
            4
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
    }
}

// solution tests ends here
