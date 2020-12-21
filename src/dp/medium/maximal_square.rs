use std::cmp::min;
/**
221. 最大正方形

https://leetcode-cn.com/problems/maximal-square/

在一个由 '0' 和 '1' 组成的二维矩阵内，找到只包含 '1' 的最大正方形，并返回其面积。

示例 1:
输入：matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
输出：4

示例 2:
输入：matrix = [["0","1"],["1","0"]]
输出：1

示例 3:
输入：matrix = [["0"]]
输出：0
**/
pub struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut dp: Vec<Vec<i32>> = Vec::new();
        let mut max = 0;
        for (row_index, row) in matrix.clone().into_iter().enumerate() {
            let mut current_row: Vec<i32> = Vec::new();
            for (col_index, col) in row.into_iter().enumerate(){
                let mut tmp = 0;
                if col == '1' {
                    if row_index == 0 || col_index == 0 {
                        tmp = 1
                    }else{
                        tmp = Self::min_three(dp[row_index-1][col_index], current_row[col_index-1], dp[row_index-1][col_index-1]) + 1
                    }
                }else{
                    tmp = 0
                }
                if tmp >= max {
                    max = tmp
                }
                current_row.push(tmp)
            }
            dp.push(current_row)
        }
        max * max
    }

    #[inline]
    fn min_three(a: i32, b: i32, c: i32) -> i32 {
        return min(a, min(b, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        matrix: Vec<Vec<char>>,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                matrix: vec![
                    vec!['1', '0', '1', '0', '0'],
                    vec!['1', '0', '1', '1', '1'],
                    vec!['1', '1', '1', '1', '1'],
                    vec!['1', '0', '0', '1', '0']
                ],
                result: 4,
            },
            Case {
                matrix: vec![
                    vec!['0', '1'],
                    vec!['1', '0']
                ],
                result: 1,
            },
            Case {
                matrix: vec![
                    vec!['0']
                ],
                result: 0,
            },
        ];
        for case_item in cases.iter() {
            let result = Solution::maximal_square(case_item.matrix.clone());
            assert_eq!(result, case_item.result)
        }
    }
}