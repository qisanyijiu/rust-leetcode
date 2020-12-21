use std::cmp::max;

/**
198. 打家劫舍

https://leetcode-cn.com/problems/house-robber/

你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。

给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。

示例 1：

输入：[1,2,3,1]
输出：4
解释：偷窃 1 号房屋 (金额 = 1) ，然后偷窃 3 号房屋 (金额 = 3)。
     偷窃到的最高金额 = 1 + 3 = 4 。
示例 2：

输入：[2,7,9,3,1]
输出：12
解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
     偷窃到的最高金额 = 2 + 9 + 1 = 12 。
**/
pub struct Solution {}

impl Solution {
    // 动态规划+滚动数组
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }

        if nums.len() == 1 {
            return nums[0]
        }

        if nums.len() == 2 {
            return max(nums[1], nums[0])
        }

        let mut prev = nums[0];
        let mut current = max(nums[1], nums[0]);
        let mut i: usize = 2;
        while i < nums.len() {
            let tmp = max(prev + nums[i], current);
            prev = current;
            current = tmp;
            i += 1
        }

        current
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
                nums: vec![1,2,3,1],
                result: 4
            },
            Case{
                nums: vec![2,7,9,3,1],
                result: 12
            },
            Case{
                nums: vec![2,1,1,2],
                result: 4
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::rob(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}