pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let left_bound = Self::left_bound(nums.clone(), target);
        let right_bound = Self::right_bound(nums.clone(), target);
        let mut ans: i32 = 0;
        if left_bound != nums.len() && right_bound != nums.len() {
            ans = (right_bound - left_bound + 1) as i32
        }
        ans
    }

    #[inline]
    pub fn left_bound(nums: Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            }else{
                right = mid - 1;
            }

        }
        if left >= nums.len() || nums[left] != target {
            left = nums.len()
        }
        left
    }

    #[inline]
    pub fn right_bound(nums: Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] > target {
                if mid == 0 {
                    right == mid;
                    break
                }
                right = mid - 1
            }else{
                left = mid + 1
            }
        }

        if nums[right] != target {
            right = nums.len()
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        target: i32,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums: vec![5, 7, 7, 8, 8, 10],
                target: 7,
                result: 2,
            },
            Case {
                nums: vec![5, 7, 7, 8, 8, 10],
                target: 8,
                result: 2,
            },
            Case {
                nums: vec![5, 7, 7, 8, 8, 10],
                target: 6,
                result: 0,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::search(case_item.nums.clone(), case_item.target);
            assert_eq!(result, case_item.result)
        }
    }
}