/*
 * [1012] numbers-with-repeated-digits
 */

pub struct Solution {}

// solution impl starts here

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        n - Self::digit_dp(n)
    }

    fn digit_dp(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        let k = digits.len();

        let mut used: [i32; 10] = [0; 10];
        let mut total = 0;

        for i in 1..k {
            total += 9 * Self::a(9, i as i32 - 1);
        }

        for i in 0..k {
            let i = k - 1 - i;
            let num = digits[i];

            for j in (if i == k - 1 { 1 } else { 0 })..num {
                if used[j as usize] != 0 {
                    continue;
                }
                total += Self::a((10 - k + i) as i32, i as i32);
            }

            used[num as usize] += 1;
            if used[num as usize] > 1 {
                break;
            }

            if i == 0 {
                total += 1;
            }
        }

        total
    }

    fn a(a: i32, b: i32) -> i32 {
        Self::fact(a) / Self::fact(a - b)
    }

    fn fact(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }
        return n * Self::fact(n - 1);
    }
}
// solution impl ends here

// solution tests starts here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::num_dup_digits_at_most_n(20), 1);
        assert_eq!(Solution::num_dup_digits_at_most_n(100), 10);
        assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
    }
}

// solution tests ends here
