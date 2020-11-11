pub struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count: [usize;26] = [0;26];
        let mut first_index: [i32;26] = [-1;26];
        const ASCII_LOWERCASE_BASE: usize = 'a' as usize;
        for (index, ch) in s.chars().enumerate(){
            let i = ch as usize - 'a' as usize;
            count[i] += 1;
            if count[i] == 1{
                first_index[i] = index as i32
            }
        }
        let mut result = -1;
        for (index, v) in count.iter().enumerate(){
            if *v == 1 && result == -1 {
                result = first_index[index]
            }
            if *v == 1 && result != -1 && first_index[index] < result{
                result = first_index[index]
            }
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
                s: String::from("leetcode"),
                result: 0
            },
            Case{
                s: String::from("loveleetcode"),
                result: 2
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::first_uniq_char(case_item.s.clone());
            assert_eq!(result, case_item.result)
        }
    }
}