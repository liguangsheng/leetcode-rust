/*
 * [0392] is-subsequence
 */

pub struct Solution {}

// solution impl starts here

// 执行用时 : 4 ms , 在所有 Rust 提交中击败了 90.00% 的用户
// 内存消耗 : 3.3 MB , 在所有 Rust 提交中击败了 80.00% 的用户
// impl Solution {
//     pub fn is_subsequence(s: String, t: String) -> bool {
//         if s.len() == 0 {
//             return true;
//         }
//         let mut i = 0;
//         for c in t.chars() {
//             if s[i..].chars().next().unwrap() == c {
//                 i += 1;
//             }
//             if i == s.len() {
//                 return true;
//             }
//         }
//         false
//     }
// }

// 执行用时 : 4 ms , 在所有 Rust 提交中击败了 90.00% 的用户
// 内存消耗 : 3.1 MB , 在所有 Rust 提交中击败了 100.00% 的用户
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let mut s_chars = s.chars();
        let mut s_char = s_chars.next();
        for c in t.chars() {
            if s_char.unwrap() == c {
                s_char = s_chars.next();
            }
            if s_char.is_none() {
                return true;
            }
        }
        false
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
            Solution::is_subsequence("abc".to_owned(), "ahbgdc".to_owned()),
            true
        );
    }
}

// solution tests ends here
