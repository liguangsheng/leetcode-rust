/*
 * [0119] pascals-triangle-ii
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut v: Vec<i32> = (0..=row_index).map(|_| 1).collect();
        for r in 2..=row_index {
            let mut y = v[0];
            for c in 1..r {
                let x = v[c];
                v[c] += y;
                y = x;
            }
        }
        v
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn test_case1() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}

// solution tests ends here
