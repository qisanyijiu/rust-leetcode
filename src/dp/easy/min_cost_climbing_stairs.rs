use std::cmp::min;

/**
746. 使用最小花费爬楼梯

https://leetcode-cn.com/problems/min-cost-climbing-stairs/

数组的每个索引作为一个阶梯，第 i个阶梯对应着一个非负数的体力花费值 cost[i](索引从0开始)。

每当你爬上一个阶梯你都要花费对应的体力花费值，然后你可以选择继续爬一个阶梯或者爬两个阶梯。

您需要找到达到楼层顶部的最低花费。在开始时，你可以选择从索引为 0 或 1 的元素作为初始阶梯。

示例 1:

输入: cost = [10, 15, 20]
输出: 15
解释: 最低花费是从cost[1]开始，然后走两步即可到阶梯顶，一共花费15。
 示例 2:

输入: cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
输出: 6
解释: 最低花费方式是从cost[0]开始，逐个经过那些1，跳过cost[3]，一共花费6。
**/
pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let length = cost.len();

        let mut current:i32 = 0;
        let mut prev: i32 = 0;

        let mut i: usize = 2;
        while i <= length {
            let next = min(current + cost[i-1], prev+cost[i-2]);
            prev = current;
            current = next;
            i += 1
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        cost: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                cost: vec![10,15,20],
                result: 15
            },
            Case{
                cost: vec![1,100,1,1,1,100,1,1,100,1],
                result: 6
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::min_cost_climbing_stairs(case_item.cost.clone());
            assert_eq!(result, case_item.result)
        }
    }
}