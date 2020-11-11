pub struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut sum = 0;
        let mut left_sum = 0;
        for num in nums.iter() {
            sum += *num;
        }
        for (index, num) in nums.iter().enumerate() {
            if left_sum == sum - left_sum - *num {
                return index as i32;
            }
            left_sum += *num;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums: vec![1, 7, 3, 6, 5, 6],
                result: 3,
            },
            Case {
                nums: vec![1, 2, 3],
                result: -1,
            },
            Case {
                nums: vec![1, 2, 3, -1, 4],
                result: 2,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::pivot_index(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}