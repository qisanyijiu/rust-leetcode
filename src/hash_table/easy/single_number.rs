pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut nums_map:HashMap<i32, i32> = HashMap::new();
        for num in nums.iter(){
            if nums_map.contains_key(num){
                let cnt = nums_map.get(num).unwrap();
                nums_map.insert(*num, cnt+1);
            } else{
                nums_map.insert(*num, 1);
            }
        }
        let mut result:i32 = 0;
        for (num, cnt) in nums_map.iter(){
            if *cnt == 1{
                result = *num;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                nums: vec![2,2,1],
                result: 1
            },
            Case{
                nums: vec![4,1,2,1,2],
                result: 4
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::single_number(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}