pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;
        while left < right {
            max_area = max_area.max((height[left].min(height[right])) * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_area
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
                height: vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
                result: 49,
            }
        ];
        for case in cases {
            let result = Solution::max_area(case.height);
            assert_eq!(result, case.result);
        }
    }
}