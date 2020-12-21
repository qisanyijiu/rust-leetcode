pub struct Solution {}
/**
139. 单词拆分

https://leetcode-cn.com/problems/word-break/

给定一个非空字符串 s 和一个包含非空单词的列表 wordDict，判定 s 是否可以被空格拆分为一个或多个在字典中出现的单词。

说明：

拆分时可以重复使用字典中的单词。
你可以假设字典中没有重复的单词。
示例 1：

输入: s = "leetcode", wordDict = ["leet", "code"]
输出: true
解释: 返回 true 因为 "leetcode" 可以被拆分成 "leet code"。
示例 2：

输入: s = "applepenapple", wordDict = ["apple", "pen"]
输出: true
解释: 返回 true 因为 "applepenapple" 可以被拆分成 "apple pen apple"。
     注意你可以重复使用字典中的单词。
示例 3：

输入: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
输出: false
**/
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {

        let mut dp: Vec<bool> = Vec::new();
        let mut i = 0;
        while i <= s.len() {
            dp.push(false);
            i += 1
        }

        dp[0] = true;
        i = 0;
        let chars: Vec<char> = s.chars().collect();
        while i < s.len() {
            let mut j = i + 1;
            while j < s.len() + 1 {
                let sub_s: String = chars[i..j].to_vec().into_iter().collect();
                if dp[i] && word_dict.contains(&sub_s) {
                    dp[j] = true
                }
                j += 1
            }
            i += 1
        }

        dp[dp.len()-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        s: String,
        word_dict: Vec<String>,
        result: bool
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                s: "leetcode".to_string(),
                word_dict: vec!["leet".to_string(), "code".to_string()],
                result: true
            },
            Case{
                s: "applepenapple".to_string(),
                word_dict: vec!["apple".to_string(), "pen".to_string()],
                result: true
            },
            Case{
                s: "catsandog".to_string(),
                word_dict: vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()],
                result: false
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::word_break(case_item.s.clone(), case_item.word_dict.clone());
            assert_eq!(result, case_item.result)
        }
    }
}