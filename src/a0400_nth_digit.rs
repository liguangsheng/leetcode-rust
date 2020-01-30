/*
 * [0400] nth-digit
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n: u32 = n as u32;
        let mut x: u32 = 0;
        let mut right: u32 = 0;

        // 确定数字区间
        while n > right {
            x += 1;
            n -= right;
            right = 10u32.pow(x - 1) * x * 9;
        }

        // 取出数字
        ((10u32.pow(x - 1) + (n - 1) / x) / 10u32.pow(if n % x > 0 { x - n % x } else { 0 }) % 10)
            as i32
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::find_nth_digit(3), 3);
        assert_eq!(Solution::find_nth_digit(10), 1);
        assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(12), 1);
        assert_eq!(Solution::find_nth_digit(1000), 3);
    }
}

// solution tests ends here
