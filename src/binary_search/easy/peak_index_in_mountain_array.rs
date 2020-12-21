/**
852. 山脉数组的峰顶索引

https://leetcode-cn.com/problems/peak-index-in-a-mountain-array/

我们把符合下列属性的数组 A 称作山脉：

A.length >= 3
存在 0 < i < A.length - 1 使得A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]
给定一个确定为山脉的数组，返回任何满足 A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1] 的 i 的值。

 

示例 1：

输入：[0,1,0]
输出：1
示例 2：

输入：[0,2,1,0]
输出：1
**/
pub struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = arr.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < arr[mid + 1] {
                left = mid + 1
            } else {
                right = mid
            }
        }

        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        arr: Vec<i32>,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                arr: vec![0, 1, 0],
                result: 1,
            },
            Case {
                arr: vec![0, 2, 1, 0],
                result: 1,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::peak_index_in_mountain_array(case_item.arr.clone());
            assert_eq!(result, case_item.result)
        }
    }
}