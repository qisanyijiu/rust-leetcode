use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        if pattern.len() != str.split_whitespace().count() {
            return false
        }
        const ASCII_LOWERCASE_BASE: usize = 'a' as usize;
        let mut bindings: [Option<String>; 26] = Default::default();
        let mut used_words = HashSet::new();

        for (ch, word) in pattern.chars().zip(str.split_whitespace()) {
            let ch_index = ch as usize - ASCII_LOWERCASE_BASE;
            match &bindings[ch_index] {
                Some(binding) if binding != word => {
                    return false
                },
                None if used_words.contains(word) => {
                    return false
                },
                None => {
                    bindings[ch_index] = Some(String::from(word));
                    used_words.insert(word);
                },
                _ => {}
            }

        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        pattern: String,
        str: String,
        result: bool
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                pattern: String::from("abba"),
                str: String::from("dog cat cat dog"),
                result: true
            },
            Case{
                pattern: String::from("abba"),
                str: String::from("dog cat cat fish"),
                result: false
            },
            Case{
                pattern: String::from("aaaa"),
                str: String::from("dog cat cat dog"),
                result: false
            },
            Case{
                pattern: String::from("abba"),
                str: String::from("dog dog dog dog"),
                result: false
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::word_pattern(case_item.pattern.clone(), case_item.str.clone());
            assert_eq!(result, case_item.result)
        }
    }
}