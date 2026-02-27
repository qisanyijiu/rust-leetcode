use crate::array::hard;

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_left = 0;
        let mut max_right = vec![0; height.len()];
        for i in (0..height.len()-1).rev() {
            max_right[i] = height[i+1].max(max_right[i+1]);
        }
        for i in 1..height.len() {
            max_left = max_left.max(height[i-1]);
            let min = max_left.min(max_right[i]);
            if min > height[i] {
                sum += min - height[i]
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        height: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                height: vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
                result: 6,
            },
            Case {
                height: vec![2, 0, 2],
                result: 2,
            }
        ];
        for case in cases {
            let result = Solution::trap(case.height);
            assert_eq!(result, case.result);
        }
    }
}