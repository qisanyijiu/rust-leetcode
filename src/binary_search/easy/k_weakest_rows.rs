/**
1337. 方阵中战斗力最弱的 K 行

https://leetcode-cn.com/problems/the-k-weakest-rows-in-a-matrix/

给你一个大小为 m * n 的方阵 mat，方阵由若干军人和平民组成，分别用 1 和 0 表示。

请你返回方阵中战斗力最弱的 k 行的索引，按从最弱到最强排序。

如果第 i 行的军人数量少于第 j 行，或者两行军人数量相同但 i 小于 j，那么我们认为第 i 行的战斗力比第 j 行弱。

军人 总是 排在一行中的靠前位置，也就是说 1 总是出现在 0 之前。

 

示例 1：

输入：mat =
[[1,1,0,0,0],
 [1,1,1,1,0],
 [1,0,0,0,0],
 [1,1,0,0,0],
 [1,1,1,1,1]],
k = 3
输出：[2,0,3]
解释：
每行中的军人数目：
行 0 -> 2
行 1 -> 4
行 2 -> 1
行 3 -> 2
行 4 -> 5
从最弱到最强对这些行排序后得到 [2,0,3,1,4]
示例 2：

输入：mat =
[[1,0,0,0],
 [1,1,1,1],
 [1,0,0,0],
 [1,0,0,0]],
k = 2
输出：[0,2]
解释：
每行中的军人数目：
行 0 -> 1
行 1 -> 4
行 2 -> 1
行 3 -> 1
从最弱到最强对这些行排序后得到 [0,2,3,1]
**/

pub struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut list:Vec<i32> = vec![];
        let mut result: Vec<i32> = vec![];

        for (index, row) in mat.into_iter().enumerate(){
            list.push(Self::count(row) * 100 + index as i32)
        }
        list.sort_unstable();
        for (index, v) in list.into_iter().enumerate() {
            if index >= k as usize {
                break
            }
            result.push(v % 100)
        }
        return result
    }

    #[inline]
    fn count(row: Vec<i32>) -> i32 {
        let mut res = 0;
        for v in row {
            res += v
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        mat: Vec<Vec<i32>>,
        k :i32,
        result: Vec<i32>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                mat: vec![
                    vec![1,1,0,0,0],
                    vec![1,1,1,1,0],
                    vec![1,0,0,0,0],
                    vec![1,1,0,0,0],
                    vec![1,1,1,1,1],
                ],
                k: 3,
                result: vec![2,0,3]
            },
            Case{
                mat: vec![
                    vec![1,0,0,0],
                    vec![1,1,1,1],
                    vec![1,0,0,0],
                    vec![1,0,0,0],
                ],
                k: 2,
                result: vec![0,2]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::k_weakest_rows(case_item.mat.clone(), case_item.k);
            assert_eq!(result, case_item.result)
        }
    }
}