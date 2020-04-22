/*
 * [0761] special-binary-string
 */

struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let s = s.as_bytes();
        let mut l = Vec::new();
        let mut start = 0;
        let mut count = 0;
        for i in 0..s.len() {
            count += if s[i] as char == '1' { 1 } else { -1 };
            if count == 0 {
                let substr = std::str::from_utf8(&s[start + 1..i]).unwrap().to_string();
                l.push("1".to_string() + &Self::make_largest_special(substr) + "0");
                start = i + 1;
            }
        }
        l.sort();
        l.reverse();
        l.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(
            Solution::make_largest_special("11011000".to_owned()),
            "11100100".to_owned()
        );
    }
}
