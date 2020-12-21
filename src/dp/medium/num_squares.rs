use std::cmp::min;

pub struct Solution {}
/**
279. 完全平方数

https://leetcode-cn.com/problems/perfect-squares/

给定正整数 n，找到若干个完全平方数（比如 1, 4, 9, 16, ...）使得它们的和等于 n。你需要让组成和的完全平方数的个数最少。

示例 1:

输入: n = 12
输出: 3
解释: 12 = 4 + 4 + 4.
示例 2:

输入: n = 13
输出: 2
解释: 13 = 4 + 9.
**/
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut square_nums: Vec<usize> = Vec::new();
        for i in 0..{
            let num = i * i;
            square_nums.push(num);
            if num > n as usize {
                break
            }
        }
        let mut dp: Vec<i32> = Vec::new();
        dp.push(0);
        for i in 1..(n + 1) as usize {
            dp.push(1 << 30);
            for square in &square_nums {
                if i < *square{
                    break
                }
                dp[i] = min(dp[i], dp[i-*square] + 1)
            }
        }
        dp[dp.len()-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        n: i32,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                n: 12,
                result: 3
            },
            Case{
                n: 13,
                result: 2
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::num_squares(case_item.n);
            assert_eq!(result, case_item.result)
        }
    }
}