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
        if s.len() < 1 {
            return s;
        }

        let s_chars: Vec<char>  = s.chars().collect();
        let mut dp: Vec<Vec<bool>> = Vec::new();
        let mut i = 0;
        while i < s.len() {
            let mut row = Vec::new();
            let mut j:usize = 0;
            while j < s.len() {
                row.push(false);
                j += 1
            }
            dp.push(row);
            i += 1
        }
        let mut ans:Vec<char> = Vec::new();

        let mut l: usize = 0;
        while l < s.len() {
            let mut i: usize = 0;
            while i + l < s.len() {
                let j = i + l;
                if l == 0 {
                    dp[i][j] = true;
                } else if l == 1 {
                    dp[i][j] = s_chars[i] == s_chars[j]
                } else{
                    dp[i][j] = s_chars[i] == s_chars[j] && dp[i+1][j-1]
                }
                if dp[i][j] && l + 1 > ans.len() {
                    ans = s_chars[i..(j+1)].to_vec()
                }
                i += 1
            }
            l += 1
        }

        let res: String = ans.into_iter().collect();
        res
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
                result: String::from("bab")
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