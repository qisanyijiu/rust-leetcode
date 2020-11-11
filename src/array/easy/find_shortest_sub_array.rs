pub struct Solution {}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        // 计算出数组的degree
        use std::collections::HashMap;
        use std::cmp;
        let mut left:HashMap<i32, usize> = HashMap::new();
        let mut right:HashMap<i32, usize> = HashMap::new();
        let mut count:HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate(){
            if !left.contains_key(num){
                left.insert(*num, i);
            }
            if !count.contains_key(num){
                count.insert(*num, 1);
            }else{
                count.insert(*num, count.get(num).unwrap()+1);
            }
            right.insert(*num, i);
        }
        let degree = count.values().max().unwrap();
        let mut result = nums.len();
        for num in count.keys(){
            if count[num] == *degree {
                result = cmp::min(result, right[num]-left[num]+1)
            }
        }
        result as i32
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
                nums: vec![1,2,2,3,1],
                result: 2
            },
            Case{
                nums: vec![1,2,2,3,1,4,2],
                result: 6
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_shortest_sub_array(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}