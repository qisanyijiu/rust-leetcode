pub struct Solution {}


impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        use std::cmp;
        let mut min1 = 2147483647;
        let mut min2 = 2147483647;
        let mut max1 = -2147483648;
        let mut max2 = -2147483648;
        let mut max3 = -2147483648;

        for n in nums.iter() {
            if n <= &min1 {
                min2 = min1;
                min1 = *n;
            } else if n <= &min2 {
                min2 = *n;
            }
            if n >= &max1 {
                max3 = max2;
                max2 = max1;
                max1 = *n;
            } else if n >= &max2 {
                max3 = max2;
                max2 = *n;
            } else if n >= &max3 {
                max3 = *n;
            }
        }
        cmp::max(min1 * min2 * max1, max1 * max2 * max3)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        input: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                input: vec![1, 2, 3],
                result: 6
            },
            Case{
                input: vec![1, 2, 3, 4],
                result: 24
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::maximum_product(case_item.input.clone());
            assert_eq!(result, case_item.result)
        }
    }
}