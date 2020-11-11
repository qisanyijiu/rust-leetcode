pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut buffer: Vec<usize> = vec![0; 512];
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        for i in 0..s.len() {
            if buffer[s_chars[i] as usize] != buffer[t_chars[i] as usize + 256] {
                return false;
            }
            buffer[s_chars[i] as usize] = i+1;
            buffer[t_chars[i] as usize + 256] = i+1;
        }
        true
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
                s: String::from("egg"),
                t: String::from("add"),
                result: true,
            },
            Case{
                s: String::from("foo"),
                t: String::from("bar"),
                result: false
            },
            Case{
                s: String::from("paper"),
                t: String::from("title"),
                result: true
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::is_isomorphic(case_item.s.clone(), case_item.t.clone());
            assert_eq!(result, case_item.result)
        }
    }
}