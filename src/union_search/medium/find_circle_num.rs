pub struct Solution {}

/**
547. 省份数量

https://leetcode-cn.com/problems/number-of-provinces/

有 n 个城市，其中一些彼此相连，另一些没有相连。如果城市 a 与城市 b 直接相连，且城市 b 与城市 c 直接相连，那么城市 a 与城市 c 间接相连。

省份 是一组直接或间接相连的城市，组内不含其他没有相连的城市。

给你一个 n x n 的矩阵 isConnected ，其中 isConnected[i][j] = 1 表示第 i 个城市和第 j 个城市直接相连，而 isConnected[i][j] = 0 表示二者不直接相连。

返回矩阵中 省份 的数量。
**/
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let length = is_connected.len();
        let mut parents = vec![-1; length];
        let mut rank = vec![0;length];

        for (i, row) in is_connected.into_iter().enumerate() {
            for (j, v) in row.into_iter().enumerate() {
                if v == 1 && i != j {
                    Self::union(&mut parents, i, j, &mut rank)
                }
            }
        }

        let mut count = 0;
        for v in parents {
            if v == - 1 {
                count += 1
            }
        }
        count
    }

    #[inline]
    fn union(parents:&mut Vec<i32>, x: usize, y: usize, rank: &mut Vec<i32>) {
        let x_root = Self::find(parents, x);
        let y_root = Self::find(parents, y);

        if x_root == y_root {
            return;
        }

        if rank[x_root as usize] > rank[y_root as usize] {
            parents[y_root as usize] = x_root;
        } else if rank[y_root as usize] > rank[x_root as usize]{
            parents[x_root as usize] = y_root;
        } else {
            parents[x_root as usize] = y_root;
            rank[y as usize] += 1
        }
    }

    #[inline]
    fn find(parents:&mut Vec<i32>, i: usize) -> i32 {
        if -1 == parents[i as usize] {
            return i as i32
        }
        Self::find(parents, i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        is_connected: Vec<Vec<i32>>,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                is_connected: vec![
                    vec![1, 1, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 1]
                ],
                result: 2,
            },
            Case {
                is_connected: vec![
                    vec![1, 0, 0],
                    vec![0, 1, 0],
                    vec![0, 0, 1]
                ],
                result: 3,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::find_circle_num(case_item.is_connected.clone());
            assert_eq!(result, case_item.result)
        }
    }
}