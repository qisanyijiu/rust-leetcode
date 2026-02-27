pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
       let mut set = vec![0;256];
       let mut ans = 0;
       let mut left = 0;
       for (i, char) in s.chars().enumerate() {
            if set[char as usize] > 0 {
                left = left.max(set[char as usize]);
            }
            set[char as usize] = i+1;
            ans = ans.max(i - left);
       }
       ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        s: String,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                s: String::from("abcabcbb"),
                result: 3,
            },
            Case {
                s: String::from("bbbbb"),
                result: 1,
            },
            Case {
                s: String::from("pwwkew"),
                result: 3,
            }
        ];
        for case in cases {
            let result = Solution::length_of_longest_substring(case.s);
            assert_eq!(result, case.result);
        }
    }
}