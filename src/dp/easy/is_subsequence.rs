pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let n = s.len();
        let m = t.len();

        let mut i: usize = 0;
        let mut j: usize = 0;

        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        while i < n && j < m  {
            if s_chars[i] == t_chars[j] {
                i += 1
            }
            j += 1
        }

        return i == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        s: String,
        t: String,
        result: bool
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                s: String::from("abc"),
                t: String::from("ahbgdc"),
                result: true,
            },
            Case{
                s: String::from("axc"),
                t: String::from("ahbgdc"),
                result: false,
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::is_subsequence(case_item.s.clone(), case_item.t.clone());
            assert_eq!(result, case_item.result)
        }
    }
}