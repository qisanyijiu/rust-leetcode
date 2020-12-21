/**
349. 两个数组的交集

https://leetcode-cn.com/problems/intersection-of-two-arrays/

给定两个数组，编写一个函数来计算它们的交集。

 

示例 1：

输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]
示例 2：

输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
**/
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut m1: HashSet<_> = nums1.iter().cloned().collect();
        let mut m2: HashSet<_> = nums2.iter().cloned().collect();

        let res : Vec<i32> = m1.intersection(&m2).cloned().collect();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        result: Vec<i32>,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums1: vec![1, 2, 2, 1],
                nums2: vec![2, 2],
                result: vec![2],
            },
            Case {
                nums1: vec![4, 9, 5],
                nums2: vec![9, 4, 9, 8, 4],
                result: vec![9, 4],
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::intersection(case_item.nums1.clone(), case_item.nums2.clone());
            assert_eq!(result, case_item.result)
        }
    }
}
