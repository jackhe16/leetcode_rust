/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut record = vec![0; 26];

        let base_char = 'a';

        for byte in s.bytes() {
            record[byte as usize - base_char as usize] += 1;
        }
        for byte in t.bytes() {
            record[byte as usize - base_char as usize] -= 1;
        }

        record.iter().filter(|x| **x != 0).count() == 0
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
    }
}
