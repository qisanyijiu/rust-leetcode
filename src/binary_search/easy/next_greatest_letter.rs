/**
744. 寻找比目标字母大的最小字母

https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target/

给你一个排序后的字符列表 letters ，列表中只包含小写英文字母。另给出一个目标字母 target，请你寻找在这一有序列表里比目标字母大的最小字母。

在比较时，字母是依序循环出现的。举个例子：

如果目标字母 target = 'z' 并且字符列表为 letters = ['a', 'b']，则答案返回 'a'


示例：

输入:
letters = ["c", "f", "j"]
target = "a"
输出: "c"

输入:
letters = ["c", "f", "j"]
target = "c"
输出: "f"

输入:
letters = ["c", "f", "j"]
target = "d"
输出: "f"

输入:
letters = ["c", "f", "j"]
target = "g"
输出: "j"

输入:
letters = ["c", "f", "j"]
target = "j"
输出: "c"

输入:
letters = ["c", "f", "j"]
target = "k"
输出: "c"
**/
pub struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut left: usize = 0;
        let mut right: usize = letters.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if letters[mid] <= target {
                left = mid + 1;
            }else{
                right = mid;
            }
        }

        return letters[left % letters.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        letters: Vec<char>,
        target: char,
        result: char,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                letters: vec!['c', 'f', 'j'],
                target: 'a',
                result: 'c',
            },
            Case {
                letters: vec!['c', 'f', 'j'],
                target: 'c',
                result: 'f',
            },
            Case {
                letters: vec!['c', 'f', 'j'],
                target: 'd',
                result: 'f',
            },
            Case {
                letters: vec!['c', 'f', 'j'],
                target: 'g',
                result: 'j',
            },
            Case {
                letters: vec!['c', 'f', 'j'],
                target: 'j',
                result: 'c',
            },
            Case {
                letters: vec!['c', 'f', 'j'],
                target: 'k',
                result: 'c',
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::next_greatest_letter(case_item.letters.clone(), case_item.target);
            assert_eq!(result, case_item.result)
        }
    }
}