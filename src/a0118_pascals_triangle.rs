/*
 * [0118] pascals-triangle
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut t = Vec::with_capacity(num_rows as usize);
        for i in 0..num_rows as usize {
            let mut v: Vec<i32> = (0..i + 1).map(|_| 1).collect();
            if i > 0 {
                for j in 1..i {
                    let last_row: &Vec<i32> = &t[i - 1];
                    v[j] = last_row[j - 1] + last_row[j];
                }
            }
            t.push(v);
        }
        t
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }

    #[test]
    fn test_case1() {
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::generate(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1]]
        );
    }
}

// solution tests ends here
