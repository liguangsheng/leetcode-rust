/*
 * [1047] remove-all-adjacent-duplicates-in-string
 */

pub struct Solution {}

// solution impl starts here

use std::collections::VecDeque;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut q = VecDeque::new();
        for c in s.chars() {
            if !q.is_empty() && q.back().unwrap() == &c {
                q.pop_back();
            } else {
                q.push_back(c);
            }
        }
        q.into_iter().collect()
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca");
    }
}

// solution tests ends here
