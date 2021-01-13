pub struct Solution {}

/**
684. 冗余连接

https://leetcode-cn.com/problems/redundant-connection/solution/rust-by-rui2o2o-tl77/

输入一个图，该图由一个有着N个节点 (节点值不重复1, 2, ..., N) 的树及一条附加的边构成。附加的边的两个顶点包含在1到N中间，这条附加的边不属于树中已存在的边。

结果图是一个以边组成的二维数组。每一个边的元素是一对[u, v] ，满足 u < v，表示连接顶点u 和v的无向图的边。

返回一条可以删去的边，使得结果图是一个有着N个节点的树。如果有多个答案，则返回二维数组中最后出现的边。答案边 [u, v] 应满足相同的格式 u < v。

示例 1：

输入: [[1,2], [1,3], [2,3]]
输出: [2,3]
解释: 给定的无向图为:
  1
 / \
2 - 3
示例 2：

输入: [[1,2], [2,3], [3,4], [1,4], [1,5]]
输出: [1,4]
解释: 给定的无向图为:
5 - 1 - 2
    |   |
    4 - 3
**/
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parents = vec![0; n + 1];
        (1..=n).for_each(|i| parents[i] = i);
        for i in 0..n {
            let mut a = edges[i][0] as usize;
            let mut b = edges[i][1] as usize;
            while parents[a] != a { a = parents[a]; }
            while parents[b] != b { b = parents[b]; }
            if a != b { parents[b] = a; } else { return edges[i].clone() }
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        edges: Vec<Vec<i32>>,
        result: Vec<i32>,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                edges: vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 3],
                ],
                result: vec![2, 3],
            },
            Case {
                edges: vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![1, 4],
                    vec![1, 5],
                ],
                result: vec![1, 4],
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::find_redundant_connection(case_item.edges.clone());
            assert_eq!(result, case_item.result)
        }
    }
}