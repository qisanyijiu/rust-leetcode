use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_cnt = HashMap::new();
        pre_cnt.insert(0, 1);

        let mut pre = 0;
        let mut res = 0;

        for num in nums {
            pre += num;
            if let Some(&cnt) = pre_cnt.get(&(pre - k)) {
                res += cnt;
            }
            *pre_cnt.entry(pre).or_insert(0) += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        k: i32,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums: vec![1, 2, 3, 4, 5],
                k: 3,
                result: 2,
            },
            Case {
                nums: vec![1, 1, 1],
                k: 2,
                result: 2,
            }
        ];

        for case in cases {
            let result = Solution::subarray_sum(case.nums, case.k);
            assert_eq!(result, case.result);
        }
    }
}   