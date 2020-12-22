pub struct Solution {}

/**
63. 不同路径 II

https://leetcode-cn.com/problems/unique-paths-ii/

一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。

现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？

示例 1：

输入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
输出：2
解释：
3x3 网格的正中间有一个障碍物。
从左上角到右下角一共有 2 条不同的路径：
1. 向右 -> 向右 -> 向下 -> 向下
2. 向下 -> 向下 -> 向右 -> 向右

示例 2：
输入：obstacleGrid = [[0,1],[0,0]]
输出：1
**/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut cache = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {  // 如果这一点有障碍物，则路径数为0
                    cache[i][j] = 0;
                } else if i == 0 && j == 0 {   // 如果是初始值，则路径数为1
                    cache[i][j] = 1;
                } else if i == 0 {
                    cache[i][j] = cache[i][j-1];
                } else if j == 0 {
                    cache[i][j] = cache[i-1][j];
                } else {
                    cache[i][j] = cache[i-1][j] + cache[i][j-1];
                }
            }
        }
        return cache[m-1][n-1];
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
                    vec![0,0,0],
                    vec![0,1,0],
                    vec![0,0,0]
                ],
                result: 2
            },
            Case{
                grid: vec![
                    vec![0, 1],
                    vec![0, 0]
                ],
                result: 1
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::unique_paths_with_obstacles(case_item.grid.clone());
            assert_eq!(result, case_item.result)
        }
    }
}