pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut char_count:HashMap<char, i32> = HashMap::new();
        for ch in s.chars(){
            match char_count.get(&ch){
                Some(cnt) => char_count.insert(ch, *cnt+1),
                None => char_count.insert(ch, 1)
            };
        }
        let mut result:i32 = 0;
        let mut flag: bool = false;
        for cnt in char_count.values(){
            if *cnt % 2 == 0{
                result += *cnt;
            }else{
                result += *cnt - 1;
                flag = true;
            }
        }
        if flag {
            result+=1
        }
        result
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
            Case{
                s: String::from("abccccdd"),
                result: 7
            },
            Case{
                s: String::from("aaabbcccc"),
                result: 9
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::longest_palindrome(case_item.s.clone());
            assert_eq!(result, case_item.result)
        }
    }
}