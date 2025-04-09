use std::collections::HashSet;

pub struct Solution {}

/**
* https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/description/?envType=daily-question&envId=2025-04-09
**/
impl Solution {

    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut st = HashSet::new();
        for x in nums {
            if x < k {
                return -1
            } else if x > k {
                st.insert(x);
            }
        }
        st.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        array: Vec<i32>,
        k: i32,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                array: vec![5, 2, 5, 4, 5],
                k: 2,
                result: 2,
            },
            Case {
                array: vec![2, 1, 2],
                k: 2,
                result: -1,
            },
            Case {
                array: vec![9, 7, 5, 3],
                k: 1,
                result: 4,
            },
        ];
        for case_item in cases.iter() {
            let result = Solution::min_operations(case_item.array.clone(), case_item.k);
            assert_eq!(result, case_item.result)
        }
    }
}
