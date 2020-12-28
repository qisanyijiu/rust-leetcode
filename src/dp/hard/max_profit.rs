pub struct Solution {}

/**
188. 买卖股票的最佳时机 IV

https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iv/

给定一个整数数组 prices ，它的第 i 个元素 prices[i] 是一支给定的股票在第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你最多可以完成 k 笔交易。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

示例 1：

输入：k = 2, prices = [2,4,1]
输出：2
解释：在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。
示例 2：

输入：k = 2, prices = [3,2,6,5,0,3]
输出：7
解释：在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
     随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。
**/
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = (prices.len() / 2).min(k as usize);
        let mut buy = vec![i32::MIN; k + 1];
        let mut sell = vec![0; k + 1];
        for v in prices {
            for i in 1..=k {
                buy[i] = buy[i].max(sell[i - 1] - v);
                sell[i] = sell[i].max(buy[i] + v);
            }
        }
        sell[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        k: i32,
        prices: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                k: 2,
                prices: vec![2, 4, 1],
                result: 2,
            },
            Case{
                k: 2,
                prices: vec![3,2,6,5,0,3],
                result: 7
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::max_profit(case_item.k, case_item.prices.clone());
            assert_eq!(result, case_item.result)
        }
    }
}