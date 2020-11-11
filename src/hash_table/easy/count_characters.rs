pub struct Solution {}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_count = [0;26];
        for c in chars.chars() {
            chars_count[Self::index(c)] += 1;
        }

        words.into_iter().filter(|word|{
            let mut count = [0;26];
            for c in word.chars() {
                let key = Self::index(c);
                count[key] += 1;
                if count[key] > chars_count[key] {
                    return false
                }
            }
            true
        }).map(|word| word.len()).sum::<usize>() as i32
    }

    #[inline]
    fn index(c: char) -> usize{
        c as usize - 'a' as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        words: Vec<String>,
        chars: String,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                words: vec!["cat".to_string(), "bt".to_string(), "hat".to_string(), "tree".to_string()],
                chars: "atach".to_string(),
                result: 6,
            },
            Case {
                words: vec!["hello".to_string(), "world".to_string(), "leetcode".to_string()],
                chars: "welldonehoneyr".to_string(),
                result: 10,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::count_characters(case_item.words.clone(), case_item.chars.clone());
            assert_eq!(result, case_item.result)
        }
    }
}