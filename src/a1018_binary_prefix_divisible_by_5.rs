/*
 * [1018] binary-prefix-divisible-by-5
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(a.len());
        let mut n = 0;

        for i in a {
            n <<= 1;
            n += i;
            result.push(n % 5 == 0);
            n %= 5;
        }
        result
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
            Solution::prefixes_div_by5(vec![0, 1, 1]),
            vec![true, false, false]
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1]),
            vec![false, false, false]
        );
    }
    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]),
            vec![true, false, false, false, true, false]
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1, 0, 1]),
            vec![false, false, false, false, false]
        );
    }
}

// solution tests ends here
