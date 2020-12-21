pub struct Solution {}

/**
121. 买卖股票的最佳时机

https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/

给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。

如果你最多只允许完成一笔交易（即买入和卖出一支股票一次），设计一个算法来计算你所能获取的最大利润。

注意：你不能在买入股票前卖出股票。

 

示例 1:

输入: [7,1,5,3,6,4]
输出: 5
解释: 在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
     注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
示例 2:

输入: [7,6,4,3,1]
输出: 0
解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
**/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0
        }
        let mut profit:i32 = 0;
        let mut left: i32 = prices[0];
        for price in prices{
            if price - left > profit {
                profit = price - left;
                continue
            }
            if price < left {
                left = price;
            }
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        prices: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                prices: vec![7,1,5,3,6,4],
                result: 5
            },
            Case{
                prices: vec![7,6,4,3,1],
                result: 0
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::max_profit(case_item.prices.clone());
            assert_eq!(result, case_item.result)
        }
    }
}