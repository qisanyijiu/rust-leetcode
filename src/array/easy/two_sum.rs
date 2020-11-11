use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) ->Vec<i32>{
        let mut map = HashMap::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate(){
            match map.get(&(target - num)) {
                None=>{
                    map.insert(num, index);
                }
                Some(sub_index)=>return vec![*sub_index as i32, index as i32]
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        target: i32,
        result: Vec<i32>
    }

    #[test]
    fn test(){
        let cases = vec![
            Case{
                nums: vec![2, 7, 11, 15],
                target: 9,
                result: vec![0,1]
            },
            Case{
                nums: vec![3,2,4],
                target: 6,
                result: vec![1,2]
            }
        ];

        for case_item in cases.iter(){
            let result = Solution::two_sum(case_item.nums.clone(), case_item.target);
            assert_eq!(result, case_item.result)
        }
    }
}