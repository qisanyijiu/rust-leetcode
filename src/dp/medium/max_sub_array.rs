pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum_dp = vec![0; nums.len()];
        max_sum_dp[0] = nums[0];
        for i in 1..nums.len() {
            max_sum_dp[i] = max_sum_dp[i-1].max(0) + nums[i];
        }
        *max_sum_dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums: vec![1, 2, 3, 4, 5],
                result: 15,
            }
        ];
    }
}       