pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: std::collections::HashMap<Vec<u8>, Vec<String>> = std::collections::HashMap::new();
        for s  in strs.iter() {
            let mut encoded_s = s
            .as_bytes()
            .to_vec();
            encoded_s.sort_unstable();
            map.entry(encoded_s).or_default().push(s.clone());
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        strs: Vec<String>,
        result: Vec<Vec<String>>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                strs: vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()],
                result: vec![vec!["eat".to_string(), "tea".to_string(), "ate".to_string()], vec!["tan".to_string(), "nat".to_string()], vec!["bat".to_string()]],
            }
        ];
        for case in cases {
            let result = Solution::group_anagrams(case.strs);
            assert_eq!(result, case.result);
        }
    }
}
