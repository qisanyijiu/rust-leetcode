/**
367. 有效的完全平方数

https://leetcode-cn.com/problems/valid-perfect-square/

给定一个正整数 num，编写一个函数，如果 num 是一个完全平方数，则返回 True，否则返回 False。

说明：不要使用任何内置的库函数，如  sqrt。

示例 1：

输入：16
输出：True
示例 2：

输入：14
输出：False

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/valid-perfect-square
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
**/

pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num < 2 {
            return true
        }
        let mut x = num / 2;
        while x * x > num {
            x = (x + num / x) / 2
        }
        x * x == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        num: i32,
        result: bool
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                num: 16,
                result: true,
            },
            Case{
                num: 14,
                result: false,
            },
            Case{
                num: 1,
                result: true,
            },
            Case{
                num: 101,
                result: false,
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::is_perfect_square(case_item.num);
            assert_eq!(result, case_item.result)
        }
    }
}