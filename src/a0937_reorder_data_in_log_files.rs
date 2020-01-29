/*
 * [0937] reorder-data-in-log-files
 */

pub struct Solution {}

// solution impl starts here

use std::cmp::Ordering;
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut mut_logs = logs;
        mut_logs.sort_by(|a, b| {
            let (a1, a2) = Solution::split(a);
            let (b1, b2) = Solution::split(b);
            let is_digit_a = a2.chars().next().unwrap().is_digit(10);
            let is_digit_b = b2.chars().next().unwrap().is_digit(10);

            if is_digit_a && !is_digit_b {
                return Ordering::Greater;
            }

            if !is_digit_a && is_digit_b {
                return Ordering::Less;
            }

            if is_digit_a && is_digit_b {
                return Ordering::Equal;
            }

            if a2 != b2 {
                return a2.cmp(b2);
            }

            a1.cmp(b1)
        });
        mut_logs
    }

    fn split(s: &str) -> (&str, &str) {
        let mut parts = s.splitn(2, " ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        (first, second)
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
            Solution::reorder_log_files(vec_string![
                "a1 9 2 3 1",
                "g1 act car",
                "zo4 4 7",
                "ab1 off key dog",
                "a8 act zoo"
            ]),
            vec_string![
                "g1 act car",
                "a8 act zoo",
                "ab1 off key dog",
                "a1 9 2 3 1",
                "zo4 4 7"
            ]
        );
    }

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::reorder_log_files(vec_string![
                "a1 9 2 3 1",
                "g1 act car",
                "zo4 4 7",
                "ab1 off key dog",
                "a8 act zoo",
                "a2 act car"
            ]),
            vec_string![
                "a2 act car",
                "g1 act car",
                "a8 act zoo",
                "ab1 off key dog",
                "a1 9 2 3 1",
                "zo4 4 7"
            ]
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::reorder_log_files(vec_string![
                "dig1 8 1 5 1",
                "let1 art can",
                "dig2 3 6",
                "let2 own kit dig",
                "let3 art zero"
            ]),
            vec_string![
                "let1 art can",
                "let3 art zero",
                "let2 own kit dig",
                "dig1 8 1 5 1",
                "dig2 3 6"
            ]
        );
    }
}

// solution tests ends here
