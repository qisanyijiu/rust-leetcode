use std::collections::{HashMap};

pub struct Solution {}


impl Solution {

    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen: u128 = 0;
        for (i, &num) in nums.iter().enumerate().rev() {
            let m = 1 << num;
            if seen & m != 0 {
                return i as i32 / 3 + 1
            }
            seen = seen | m;
        }
        0
    }
    pub fn minimum_operations_large(nums: Vec<i32>) -> i32 {
        let mut seen: [bool; 101] = [false; 101];
        for i in (0..nums.len()).rev() {
            let num = nums[i] as usize;
            if seen[num] {
                return i  as i32 / 3 + 1
            }
            seen[num] = true;
        }
        return 0
    }
    pub fn minimum_operations_slow(nums: Vec<i32>) -> i32 {
        let mut operation_cnt  = 0;
        let mut num_count = HashMap::new();
        let len = nums.len();
        for i in nums.iter() {
            num_count.entry(*i).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut i = 0;
        while i < len -1  {
            if num_count.len() == len - i{
                break;
            }
            operation_cnt += 1;
            if i + 3 > len {
                break;
            }
            for j in i..i+3 {
                num_count.entry(nums[j]).and_modify(|e| *e -= 1);
                if num_count[&nums[j]] == 0 {
                    num_count.remove(&nums[j]);
                }
            }

            i += 3;
        }
        operation_cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        input: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                input: vec![1,2,3,4,2,3,3,5,7],
                result: 2
            },
            Case{
                input: vec![4,5,6,4,4],
                result: 2
            },
            Case{
                input: vec![6,7,8,9],
                result: 0
            },
            Case{
                input: vec![7,8,85,64,16,9,17,10,5,80,21],
                result: 0,
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::minimum_operations(case_item.input.clone());
            assert_eq!(result, case_item.result)
        }
    }
}