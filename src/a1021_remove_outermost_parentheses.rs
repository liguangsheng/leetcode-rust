/*
 * [1021] remove-outermost-parentheses
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let (mut l, mut r) = (0, 0);
        let mut res = String::new();
        for i in s.chars() {
            if i == '(' {
                l += 1;
            } else {
                r += 1;
            }

            if l == r {
                l = 0;
                r = 0;
            } else if l > 1 {
                res.push(i);
            }
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
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()"
        );
    }
    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())"
        );
    }
    #[test]
    fn test_case2() {
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
    }
}

// solution tests ends here
