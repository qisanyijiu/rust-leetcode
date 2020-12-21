use std::cmp::max;

pub struct Solution {}

/**
面试题 17.16. 按摩师
本问题就是打家劫舍问题的变种

https://leetcode-cn.com/problems/the-masseuse-lcci/

一个有名的按摩师会收到源源不断的预约请求，每个预约都可以选择接或不接。在每次预约服务之间要有休息时间，因此她不能接受相邻的预约。给定一个预约请求序列，替按摩师找到最优的预约集合（总预约时间最长），返回总的分钟数。

注意：本题相对原题稍作改动

示例 1：

输入： [1,2,3,1]
输出： 4
解释： 选择 1 号预约和 3 号预约，总时长 = 1 + 3 = 4。
示例 2：

输入： [2,7,9,3,1]
输出： 12
解释： 选择 1 号预约、 3 号预约和 5 号预约，总时长 = 2 + 9 + 1 = 12。
示例 3：

输入： [2,1,4,5,3,1,1,3]
输出： 12
解释： 选择 1 号预约、 3 号预约、 5 号预约和 8 号预约，总时长 = 2 + 4 + 3 + 3 = 12。
**/
impl Solution {
    pub fn massage(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }

        let mut dp0 = 0;
        let mut dp1 = nums[0];

        let mut i:usize = 1;
        while i < nums.len() {
            let tmp_dp0 = max(dp0, dp1);
            let tmp_dp1 = dp0 + nums[i];

            dp0 = tmp_dp0;
            dp1 = tmp_dp1;

            i += 1;
        }

        return max(dp0, dp1)
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
                nums: vec![2,1,4,5,3,1,1,3],
                result: 12
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::massage(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}