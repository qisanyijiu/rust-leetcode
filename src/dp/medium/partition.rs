pub struct Solution {}

/**
131. 分割回文串

https://leetcode-cn.com/problems/palindrome-partitioning/

给定一个字符串 s，将 s 分割成一些子串，使每个子串都是回文串。

返回 s 所有可能的分割方案。

示例:

输入: "aab"
输出:
[
  ["aa","b"],
  ["a","a","b"]
]
**/
impl Solution {

    // TODO: understand
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res:Vec<Vec<String>> = Vec::new();
        let mut output = Vec::new();

        let s: Vec<u8> = s.chars().map(|x| {(x as u32) as u8}).collect();
        Self::palindrome_partition_dfs(&s, 0, &mut output, &mut res);

        res
    }

    fn palindrome_partition_dfs<'a>(s: &'a Vec<u8>, start: usize, output: &mut Vec<&'a [u8]>, res: &mut Vec<Vec<String>>) {
        if start == s.len() {
            res.push(output.iter().map(|&x| {
                x.iter().map(|&y| {
                    y as char
                }).collect()
            }).collect());
            return;
        }

        for i in start..s.len() {
            // 以s[start,i]为树头, 开始查找后续是否是回文, 共s.len()颗树
            if Self::is_palindrome(s, start, i) {
                if let Some(x) = s.get(start..=i) {
                    output.push(x);
                }
                Self::palindrome_partition_dfs(s, i+1, output, res);
                output.pop();
            }
        }
    }

    fn is_palindrome(s: &Vec<u8>, start: usize, end: usize) -> bool {
        let mut itr = s.iter().enumerate().skip(start)
            .zip(s.iter().enumerate().skip(start).take(1+end.saturating_sub(start)).rev());
        while let Some(x) = itr.next() {
            if (x.0).0 < (x.1).0 {
                if (x.0).1 != (x.1).1 {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        s: String,
        result: Vec<Vec<String>>,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                s: "aab".to_string(),
                result: vec![
                    vec!["a".to_string(), "a".to_string(), "b".to_string()],
                    vec!["aa".to_string(), "b".to_string()],
                ],
            },
        ];
        for case_item in cases.iter() {
            let result = Solution::partition(case_item.s.clone());
            assert_eq!(result, case_item.result)
        }
    }
}