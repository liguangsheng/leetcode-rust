/*
 * [0646] maximum-length-of-pair-chain
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_by_key(|p| p[1]);
        let mut t = pairs[0][1];
        let mut count = 1;
        for p in pairs {
            if p[0] > t {
                count += 1;
                t = p[1];
            }
        }
        count
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
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::find_longest_chain(vec![
                vec![-10, -8],
                vec![8, 9],
                vec![-5, 0],
                vec![6, 10],
                vec![-6, -4],
                vec![1, 7],
                vec![9, 10],
                vec![-4, 7]
            ]),
            4
        );
    }
}

// solution tests ends here
