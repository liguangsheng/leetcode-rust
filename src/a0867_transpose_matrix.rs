/*
 * [0867] transpose-matrix
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if a.len() == 0 {
            return vec![];
        }

        let h = a.len() as usize;
        let w = a[0].len() as usize;
        let mut res = Vec::new();

        for r in 0..w {
            let mut row = Vec::new();
            for c in 0..h {
                row.push(a[c][r]);
            }
            res.push(row);
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
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![[1, 4], [2, 5], [3, 6]]
        );
    }
}

// solution tests ends here
