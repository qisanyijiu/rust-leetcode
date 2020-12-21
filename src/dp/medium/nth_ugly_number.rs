pub struct Solution {}
/**
264. 丑数 II

https://leetcode-cn.com/problems/ugly-number-ii/

编写一个程序，找出第 n 个丑数。

丑数就是质因数只包含 2, 3, 5 的正整数。

示例:

输入: n = 10
输出: 12
解释: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 是前 10 个丑数。
**/
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        Self::calculate(n)[n as usize-1]
    }

    fn calculate(n: i32) -> Vec<i32> {
        let (mut p2, mut p3, mut p5) = (0, 0, 0);
        let mut ret: Vec<i32> = vec![1];
        for i in 1..n as usize {
            let new = (ret[p2]*2).min(ret[p3]*3).min(ret[p5]*5);
            ret.push(new);
            if ret[p2]*2 == new { p2 += 1; }
            if ret[p3]*3 == new { p3 += 1; }
            if ret[p5]*5 == new { p5 += 1; }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        n: i32,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                n: 1,
                result: 1,
            },
            Case {
                n: 2,
                result: 2,
            },
            Case {
                n: 3,
                result: 3,
            },
            Case {
                n: 4,
                result: 4,
            },
            Case {
                n: 5,
                result: 5,
            },
            Case {
                n: 6,
                result: 6,
            },
            Case {
                n: 7,
                result: 8,
            },
            Case {
                n: 8,
                result: 9,
            },
            Case {
                n: 9,
                result: 10,
            },
            Case {
                n: 10,
                result: 12,
            },
        ];
        for case_item in cases.iter() {
            let result = Solution::nth_ugly_number(case_item.n);
            assert_eq!(result, case_item.result)
        }
    }
}