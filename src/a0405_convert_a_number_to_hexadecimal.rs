/*
 * [0405] convert-a-number-to-hexadecimal
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let s = "0123456789abcdef";
        let m = 0x0000000f;
        let mut n: i64 = num as i64 & 0xffffffff;
        let mut r = String::new();
        while n > 0 {
            r.push(s.chars().nth((n & m) as usize).unwrap());
            n >>= 4;
        }
        r.chars().rev().collect::<String>()
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::to_hex(26), "1a");
    }

    #[test]
    fn test_case1() {
        assert_eq!(Solution::to_hex(-1), "ffffffff");
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::to_hex(-2), "fffffffe");
    }

    #[test]
    fn test_case3() {
        assert_eq!(Solution::to_hex(0), "0");
    }

    #[test]
    fn test_case4() {
        assert_eq!(Solution::to_hex(111111), "1b207");
    }
}

// solution tests ends here
