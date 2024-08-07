/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        for i in 2..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::fib(2), 1);
    }
}
