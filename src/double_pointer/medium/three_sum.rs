pub struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = vec![];
        for i in 0..n - 2 {
            let x = nums[i];
            if i > 0 && x == nums[i - 1] {
                // 跳过相同的数字
                continue;
            }
            if x + nums[i + 1] + nums[i + 2] > 0 {
                //如果接下来3个相加大于0，则不可能找到三数之和为0
                break;
            }
            if x + nums[n - 2] + nums[n - 1] < 0 {
                // 如果和最后两个相加都小于0，则不可能找到三数之和为0
                continue;
            }
            let mut j = i + 1;
            let mut k = n - 1;
            // 双指针扫描
            while j < k {
                let s = x + nums[j] + nums[k];
                if s > 0 {
                    // 如果和大于0，则右指针左移
                    k -= 1;
                } else if s < 0 {
                    // 如果和小于0，则左指针右移
                    j += 1;
                } else {
                    // 如果和等于0，则找到一个三数之和为0的组合
                    ans.push(vec![x, nums[j], nums[k]]);
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        // 如果左指针的前一个数字和当前数字相同，则左指针右移
                        j += 1;
                    }
                    k -= 1;
                    while k > j && nums[k] == nums[k + 1] {
                        // 如果右指针的前一个数字和当前数字相同，则右指针左移
                        k -= 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: Vec<Vec<i32>>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums: vec![-1, 0, 1, 2, -1, -4],
                result: vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            }
        ];
        for case in cases {
            let result = Solution::three_sum(case.nums);
            assert_eq!(result, case.result);
        }
    }
}