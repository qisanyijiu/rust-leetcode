pub struct Solution {}

/**
330. 按要求补齐数组

https://leetcode-cn.com/problems/patching-array/

给定一个已排序的正整数数组 nums，和一个正整数 n 。从 [1, n] 区间内选取任意个数字补充到 nums 中，使得 [1, n] 区间内的任何数字都可以用 nums 中某几个数字的和来表示。请输出满足上述要求的最少需要补充的数字个数。

示例 1:

输入: nums = [1,3], n = 6
输出: 1
解释:
根据 nums 里现有的组合 [1], [3], [1,3]，可以得出 1, 3, 4。
现在如果我们将 2 添加到 nums 中， 组合变为: [1], [2], [3], [1,3], [2,3], [1,2,3]。
其和可以表示数字 1, 2, 3, 4, 5, 6，能够覆盖 [1, 6] 区间里所有的数。
所以我们最少需要添加一个数字。
示例 2:

输入: nums = [1,5,10], n = 20
输出: 2
解释: 我们需要添加 [2, 4]。
示例 3:

输入: nums = [1,2,2], n = 5
输出: 0
**/
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let (mut max,mut cnt)=(1,0); //exclusive
        nums.into_iter().filter(|&x|x<=n).for_each(|x|if max>=x as i64{max+=x as i64}else{while x as i64>max{max<<=1;cnt+=1}max+=x as i64});
        while n as i64>=max{max<<=1;cnt+=1}
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        n: i32,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                nums: vec![1,3],
                n: 6,
                result: 1
            },
            Case{
                nums: vec![1,5,10],
                n: 20,
                result: 2
            },
            Case{
                nums: vec![1,2,2],
                n: 5,
                result: 0,
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::min_patches(case_item.nums.clone(), case_item.n);
            assert_eq!(result, case_item.result)
        }
    }
}