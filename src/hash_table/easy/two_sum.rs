use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(sub_index) => return vec![*sub_index as i32, index as i32],
                None => { map.insert(num, index); }
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
    fn test() {
        let cases = vec![
            Case {
                nums: vec![2, 7, 11, 15],
                target: 9,
                result: vec![0, 1],
            },
        ];
        for case in cases {
            let result = Solution::two_sum(case.nums, case.target);
            assert_eq!(result, case.result);
        }
    }

}