/*
 * [0779] k-th-symbol-in-grammar
 */

struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut k = k;
        let mut n = n - 1;
        let mut p = false;
        while n > 0 {
            n -= 1;
            if k % 2 == 0 {
                p = !p;
            }
            k = (k - 1) / 2 + 1;
        }
        p as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
        assert_eq!(Solution::kth_grammar(4, 5), 1);
    }
}
