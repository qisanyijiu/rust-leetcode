pub struct Solution {}

/**
70. 爬楼梯

https://leetcode-cn.com/problems/climbing-stairs/

假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

注意：给定 n 是一个正整数。

示例 1：

输入： 2
输出： 2
解释： 有两种方法可以爬到楼顶。
1.  1 阶 + 1 阶
2.  2 阶
示例 2：

输入： 3
输出： 3
解释： 有三种方法可以爬到楼顶。
1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶
**/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return n
        }
        let mut current = 2;
        let mut prev = 1;
        let mut i = 3;
        while i <= n {
            let tmp = current + prev;
            prev = current;
            current = tmp;
            i += 1;
        }
        return current
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
                n: 2,
                result: 2
            },
            Case{
                n: 3,
                result: 3
            },
            Case{
                n: 4,
                result: 5
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::climb_stairs(case_item.n);
            assert_eq!(result, case_item.result)
        }
    }
}