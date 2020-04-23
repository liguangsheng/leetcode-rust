/*
 * [1137] n-th-tribonacci-number
 */

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp: [i32; 38] = [0; 38];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;
        for i in 3..(n + 1) as usize {
            dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case0() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
        assert_eq!(Solution::tribonacci(30), 29249425);
    }
}
