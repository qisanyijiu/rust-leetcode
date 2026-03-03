pub struct Solution {}

/**
5. 最长回文子串

https://leetcode-cn.com/problems/longest-palindromic-substring/

给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

示例 1：

输入: "babad"
输出: "bab"
注意: "aba" 也是一个有效答案。
示例 2：

输入: "cbbd"
输出: "bb"
**/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let s_chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        for i in 1..s.len() {
            let len1 = Self::expand_around_center(&s_chars, i as i32, i as i32);
            let len2 = Self::expand_around_center(&s_chars, i as i32, (i+1) as i32);
            let len = len1.max(len2);
            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }
        return s[start..=end].to_string();
    }

    fn expand_around_center(s: &Vec<char>, left: i32, right: i32) -> usize {
        let mut left = left;
        let mut right = right;
        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        return (right - left - 1) as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        s: String,
        result: String
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                s: String::from("babad"),
                result: String::from("aba")
            },
            Case{
                s: String::from("cbbd"),
                result: String::from("bb")
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::longest_palindrome(case_item.s.clone());
            assert_eq!(result, case_item.result)
        }
    }
}