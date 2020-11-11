pub struct Solution {}

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result:Vec<Vec<i32>> = vec![];
        let mut start:usize = 0;
        for (index,ch) in s.as_bytes().iter().enumerate(){
            if index > 0{
                if *ch != s.as_bytes()[start]{
                    if index - start >2 {
                        result.push(vec![start as i32, (index-1) as i32])
                    }
                    start = index;
                }else if *ch == s.as_bytes()[start] && index == s.len() - 1 && index-start>1{
                    result.push(vec![start as i32, index as i32])
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        input: String,
        result: Vec<Vec<i32>>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                input: String::from("abbxxxxzzy"),
                result: vec![
                    vec![3, 6],
                ]
            },
            Case{
                input: String::from("abc"),
                result: vec![]
            },
            Case{
                input: String::from("abcdddeeeeaabbbcd"),
                result: vec![
                    vec![3,5],
                    vec![6,9],
                    vec![12,14]
                ]
            },
            Case{
                input: String::from("aaa"),
                result: vec![
                    vec![0, 2]
                ]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::large_group_positions(case_item.input.clone());
            assert_eq!(result, case_item.result)
        }
    }
}