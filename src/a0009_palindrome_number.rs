/*
 * [0009] palindrome-number
 * 判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
 *
 * 示例 1:
 *
 * 输入: 121
 * 输出: true
 * 示例 2:
 *
 * 输入: -121
 * 输出: false
 * 解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
 * 示例 3:
 *
 * 输入: 10
 * 输出: false
 * 解释: 从右向左读, 为 01 。因此它不是一个回文数。
 * 进阶:
 *
 * 你能不将整数转为字符串来解决这个问题吗？
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn is_palindrome_with_string(x: i32) -> bool {
        let x_str = x.to_string();
        let x_bytes = x_str.as_bytes();
        let (mut a, mut b) = (0, x_bytes.len() - 1);
        println!("a: {:?}, b: {:?}", a, b);
        while a <= b {
            if x_bytes[a] != x_bytes[b] {
                return false;
            };
            a += 1;
            b -= 1;
        }
        true
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let (mut p, mut n) = (x, 0i32);
        while p > n {
            n *= 10;
            n += p % 10;
            p /= 10;
        }
        p == n || p == n / 10
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}

// solution tests ends here
