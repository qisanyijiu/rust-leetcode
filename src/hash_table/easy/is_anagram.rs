pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut table: Vec<i32> = vec![0; 27];
        for ch in s.as_bytes().iter() {
            let index = (*ch - 'a' as u8) as usize;
            table[index] += 1;
        }
        for ch in t.as_bytes().iter() {
            let index = (*ch - 'a' as u8) as usize;
            table[index] -= 1;
            if table[index] < 0 {
                return false;
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
        t: String,
        result: bool,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                s: String::from("anagram"),
                t: String::from("nagaram"),
                result: true,
            },
            Case {
                s: String::from("rat"),
                t: String::from("car"),
                result: false,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::is_anagram(case_item.s.clone(), case_item.t.clone());
            assert_eq!(result, case_item.result)
        }
    }
}