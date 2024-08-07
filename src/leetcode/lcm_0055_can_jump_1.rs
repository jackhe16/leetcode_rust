/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut far = 0;
        let mut i = 0;

        while i < n && i <= far {
            if far >= n - 1 {
                return true;
            }
            far = far.max(i + nums[i] as usize);
            i += 1;
        }

        false
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::can_jump(vec![0]), true);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::can_jump(vec![1, 2]), true);
    }
}
