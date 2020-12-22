pub struct Solution {}

/**
62. 不同路径

https://leetcode-cn.com/problems/unique-paths/

一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。

问总共有多少条不同的路径？

示例 1：
输入：m = 3, n = 7
输出：28

示例 2：
输入：m = 3, n = 2
输出：3
解释：
从左上角开始，总共有 3 条路径可以到达右下角。
1. 向右 -> 向右 -> 向下
2. 向右 -> 向下 -> 向右
3. 向下 -> 向右 -> 向右

示例 3：
输入：m = 7, n = 3
输出：28

示例 4：
输入：m = 3, n = 3
输出：6
**/

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut record: Vec<i32> = vec![1; n];
        for i in 1..m {
            for j in 1..n {
                record[j] = record[j - 1] + record[j];
            }
        }
        record[n as usize- 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        m: i32,
        n: i32,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                m: 3,
                n: 7,
                result: 28,
            },
            Case {
                m: 3,
                n: 2,
                result: 3,
            },
            Case {
                m: 7,
                n: 3,
                result: 28,
            },
            Case {
                m: 3,
                n: 3,
                result: 6,
            },
        ];
        for case_item in cases.iter() {
            let result = Solution::unique_paths(case_item.m, case_item.n);
            assert_eq!(result, case_item.result)
        }
    }
}