pub struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        use std::cmp;

        let mut sum = 0;
        for (i, v) in nums.iter().enumerate(){
            if i as i32 >= k{
                break
            }
            sum += *v
        }
        let mut res = sum;
        let mut i:usize = k as usize;
        while i < nums.len() as usize{
            sum += nums[i] - nums[i-k as usize];
            res = cmp::max(res, sum);
            i += 1
        }
        return res as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        k: i32,
        result: f64
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                nums: vec![1,12,-5,-6,50,3],
                k: 4,
                result: 12.75
            },
            Case{
                nums: vec![11],
                k: 1,
                result: 11.0
            },
            Case{
                nums: vec![0,1,1,3,3],
                k: 4,
                result: 2.0
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_max_average(case_item.nums.clone(), case_item.k);
            assert_eq!(result, case_item.result)
        }
    }
}