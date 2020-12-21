use std::cmp::min;
/**
322. 零钱兑换

https://leetcode-cn.com/problems/coin-change/

给定不同面额的硬币 coins 和一个总金额 amount。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回 -1。

你可以认为每种硬币的数量是无限的。

 

示例 1：

输入：coins = [1, 2, 5], amount = 11
输出：3
解释：11 = 5 + 5 + 1
示例 2：

输入：coins = [2], amount = 3
输出：-1
示例 3：

输入：coins = [1], amount = 0
输出：0
示例 4：

输入：coins = [1], amount = 1
输出：1
示例 5：

输入：coins = [1], amount = 2
输出：2
**/
pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp: Vec<i32> = Vec::new();
        let mut i: usize = 0;
        while i <= amount as usize{
            dp.push(amount+1);
            i += 1
        }
        dp[0] = 0;
        i = 0;
        while i <= amount as usize{
            for coin in &coins {
                if *coin <= i as i32 {
                    dp[i] = min(dp[i], dp[i-(*coin as usize)] + 1)
                }
            }
            i += 1
        }

        if dp[amount as usize] > amount {
            return -1
        }
        return dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        coins: Vec<i32>,
        amount: i32,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                coins: vec![1,2,5],
                amount: 11,
                result: 3
            },
            Case{
                coins: vec![2],
                amount: 3,
                result: -1
            },
            Case{
                coins: vec![1],
                amount: 0,
                result: 0
            },
            Case{
                coins: vec![1],
                amount: 1,
                result: 1
            },
            Case{
                coins: vec![1],
                amount: 2,
                result: 2
            },
            Case{
                coins: vec![2147483647],
                amount: 2,
                result: -1
            },
            Case{
                coins: vec![1,2147483647],
                amount: 2,
                result: 2
            },
            Case{
                coins: vec![474,83,404,3],
                amount: 264,
                result: 8
            },
            Case{
                coins: vec![186,419,83,408],
                amount: 6249,
                result: 20
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::coin_change(case_item.coins.clone(), case_item.amount);
            assert_eq!(result, case_item.result)
        }
    }
}