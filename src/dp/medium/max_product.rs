use std::cmp::{max, min};

/**
152. 乘积最大子数组

https://leetcode-cn.com/problems/maximum-product-subarray/

给你一个整数数组 nums ，请你找出数组中乘积最大的连续子数组（该子数组中至少包含一个数字），并返回该子数组所对应的乘积。

 

示例 1:

输入: [2,3,-2,4]
输出: 6
解释: 子数组 [2,3] 有最大乘积 6。
示例 2:

输入: [-2,0,-1]
输出: 0
解释: 结果不能为 2, 因为 [-2,-1] 不是子数组。
**/
pub struct Solution {}

impl Solution {

    // dp + 滚动数组
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut min_f = nums[0];
        let mut max_f = nums[0];
        let mut ans = nums[0];

        let mut i = 1;
        while i < nums.len() {
            let num = nums[i];
            let mx = max_f;
            let mn = min_f;
            max_f = max(mx * num, max(num, mn * num));
            min_f = min(mn * num, min(num, mx * num));
            ans = max(max_f, ans);
            i += 1
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums: vec![2, 3, -2, 4, 3],
                result: 12,
            },
            Case {
                nums: vec![-2, 0, -1],
                result: 0,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::max_product(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}