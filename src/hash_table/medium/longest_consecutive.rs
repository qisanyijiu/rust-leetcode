pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums_set = nums.into_iter().collect::<std::collections::HashSet<i32>>();
        let mut ans: i32 = 0;
        for &num in &nums_set {
            if nums_set.contains(&(num - 1)) {
                continue;
            }
            let mut y = num;
            while nums_set.contains(&y) {
                y += 1;
            }
            ans = ans.max(y - num);
        }
        ans
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
                nums: vec![100, 4, 200, 1, 3, 2],
                result: 4,
            }
        ];
        for case in cases {
            let result = Solution::longest_consecutive(case.nums);
            assert_eq!(result, case.result);
        }
    }
}