pub struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut copy = nums.clone();
        copy.sort();
        let mut map:HashMap<i32, usize> = HashMap::new();
        for (i, v) in copy.iter().enumerate(){
            if !map.contains_key(v) {
                map.insert(*v, i);
            }
        }
        let mut result:Vec<i32> = vec![];
        for n in nums.iter(){
            result.push(map[n] as i32)
        }
        result

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: Vec<i32>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                nums: vec![8,1,2,2,3],
                result: vec![4,0,1,1,3]
            },
            Case{
                nums: vec![6,5,4,8],
                result: vec![2,1,0,3]
            },
            Case{
                nums: vec![7,7,7,7],
                result: vec![0,0,0,0]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::smaller_numbers_than_current(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}