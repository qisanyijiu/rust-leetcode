pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        while i < nums.len() {
            nums[i] = 0;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: Vec<i32>
    }

    #[test]
    fn test() {
        let mut cases = vec![
            Case {
                nums: vec![0, 1, 0, 3, 12],
                result: vec![1, 3, 12, 0, 0],
            }
        ];
        for mut case in cases {
            Solution::move_zeroes(&mut case.nums);
            assert_eq!(case.nums, case.result);
        }
    }
}