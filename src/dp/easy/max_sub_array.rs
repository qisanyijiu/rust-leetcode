pub struct Solution {}

/**
53. 最大子序和

https://leetcode-cn.com/problems/maximum-subarray/

给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

示例:

输入: [-2,1,-3,4,-1,2,1,-5,4]
输出: 6
解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
**/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp:Vec<i32> = Vec::new();
        let mut max = nums[0];
        for (i, num) in nums.into_iter().enumerate() {
            let mut tmp;
            if i == 0 || num + dp[i-1] < num {
                tmp = num;
            }else{
                tmp = num + dp[i-1];
            }

            dp.push(tmp);
            if tmp > max {
                max = tmp
            }
        }
        max
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
                nums: vec![-2,1,-3,4,-1,2,1,-5,4],
                result: 6
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::max_sub_array(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}