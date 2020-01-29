/*
 * [0007] reverse-integer
 */

pub struct Solution {}

// solution impl starts here
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut m = x.abs() as i64;
        let mut n = 0i64;
        while m > 0 {
            n *= 10;
            n += m % 10;
            m /= 10;
        }
        if x < 0 {
            n *= -1;
        }

        if n > std::i32::MAX as i64 || n < std::i32::MIN as i64 {
            return 0;
        }

        n as i32
    }
}
// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(0, Solution::reverse(1534236469));
    }
}

// solution tests ends here
