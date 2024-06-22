/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    /**
     * dp[0][i]: 第i天为持有时的最大利润
     * dp[1][i]: 第i天为不持有(非卖出,非冷冻期)时的最大利润
     * dp[2][i]: 第i天为卖出时的最大利润
     * dp[3][i]: 第i天为冷冻期时的最大利润
     *
     * dp[0][i] = max(dp[0][i - 1], dp[1][i - 1] - prices[i], dp[3][i - 1] - prices[i]);
     * dp[1][i] = max(dp[1][i - 1], dp[3][i - 1]);
     * dp[2][i] = dp[0][i - 1] + prices[i];
     * dp[3][i] = dp[2][i - 1];
     */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut dp = vec![vec![0; len]; 4];
        dp[0][0] = -prices[0];

        for i in 1..len {
            dp[0][i] = cmp::max(
                cmp::max(dp[0][i - 1], dp[1][i - 1] - prices[i]),
                dp[3][i - 1] - prices[i],
            );
            dp[1][i] = cmp::max(dp[1][i - 1], dp[3][i - 1]);
            dp[2][i] = dp[0][i - 1] + prices[i];
            dp[3][i] = dp[2][i - 1];
        }

        dp[1][len - 1].max(dp[2][len - 1]).max(dp[3][len - 1])
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
