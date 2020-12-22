use std::cmp::min;

pub struct Solution {}

/**
64. 最小路径和

https://leetcode-cn.com/problems/minimum-path-sum/

给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

示例 1：
输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
输出：7
解释：因为路径 1→3→1→1→1 的总和最小。

示例 2：
输入：grid = [[1,2,3],[4,5,6]]
输出：12
**/
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut sum_grid: Vec<Vec<i32>> = Vec::new();
        for (row_index, row) in grid.into_iter().enumerate() {
            let mut current_row: Vec<i32> = Vec::new();
            for (col_index, col) in row.into_iter().enumerate() {
                if row_index == 0 && col_index == 0 {
                    current_row.push(col)
                }else if row_index == 0 {
                    current_row.push(col + current_row[col_index-1])
                }else if col_index == 0 {
                    current_row.push(col + sum_grid[row_index-1][0])
                }else{
                    current_row.push(col + min(current_row[col_index-1], sum_grid[row_index-1][col_index]))
                }
            }
            sum_grid.push(current_row)
        }
        let row_index = sum_grid.len() - 1;
        let col_index = sum_grid[0].len() - 1;
        sum_grid[row_index][col_index]
    }
 }

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        grid: Vec<Vec<i32>>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                grid: vec![
                    vec![1,3,1],
                    vec![1,5,1],
                    vec![4,2,1],
                ],
                result: 7
            },
            Case{
                grid: vec![
                    vec![1,2,3],
                    vec![4,5,6],
                ],
                result: 12
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::min_path_sum(case_item.grid.clone());
            assert_eq!(result, case_item.result)
        }
    }
}