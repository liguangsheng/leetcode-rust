/*
 * [0343] integer-break
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let a = n / 3;
        let b = n % 3;
        if b == 0 {
            return 3i32.pow(a as u32);
        }

        if b == 1 {
            return 3i32.pow(a as u32 - 1) * 4;
        }

        3i32.pow(a as u32) * 2
    }
}

// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}

// solution tests ends here
