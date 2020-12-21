/**
1351. 统计有序矩阵中的负数

https://leetcode-cn.com/problems/count-negative-numbers-in-a-sorted-matrix/

给你一个 m * n 的矩阵 grid，矩阵中的元素无论是按行还是按列，都以非递增顺序排列。 

请你统计并返回 grid 中 负数 的数目。

 

示例 1：

输入：grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
输出：8
解释：矩阵中共有 8 个负数。
示例 2：

输入：grid = [[3,2],[1,0]]
输出：0
示例 3：

输入：grid = [[1,-1],[-1,-1]]
输出：3
示例 4：

输入：grid = [[-1]]
输出：1
**/

pub struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut num: i32 = 0;
        for row in grid {
            let mut left: i32 = 0;
            let mut right: i32 = (row.len() - 1) as i32;
            let mut pos:i32 = (row.len() + 2) as i32;

            while left <= right {
                let mid = left + (right-left) / 2;
                if row[mid as usize] < 0 {
                    pos = mid as i32;
                    right = mid - 1;
                }else{
                    left = mid + 1
                }
            }

            if pos != (row.len() + 2) as i32 {
                num += row.len() as i32 - pos
            }
        }
        num

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        grid: Vec<Vec<i32>>,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                grid: vec![
                    vec![4, 3, 2, -1],
                    vec![3, 2, 1, -1],
                    vec![1, 1, -1, -2],
                    vec![-1, -1, -2, -3]
                ],
                result: 8,
            },
            Case {
                grid: vec![
                    vec![3, 2],
                    vec![1, 0]
                ],
                result: 0,
            },
            Case {
                grid: vec![
                    vec![1, -1],
                    vec![-1, -1]
                ],
                result: 3,
            },
            Case {
                grid: vec![vec![-1]],
                result: 1,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::count_negatives(case_item.grid.clone());
            assert_eq!(result, case_item.result)
        }
    }
}