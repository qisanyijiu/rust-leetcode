pub struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut num_counts = HashMap::new();
        let (short_nums, long_nums) = if nums1.len() > nums2.len() {
            (&nums2, &nums1)
        } else {
            (&nums1, &nums2)
        };
        for &num in short_nums {
            let count = num_counts.entry(num).or_insert(0);
            *count += 1;
        }
        let mut intersect = Vec::new();
        for num in long_nums {
            if let Some(count) = num_counts.get_mut(num) {
                if *count > 0 {
                    *count -= 1;
                    intersect.push(*num);
                }
            }
        }
        intersect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        result: Vec<i32>
    } 
    
    #[test]
    fn test() {
        let cases = vec![
            Case{
                nums1: vec![1,2,2,1],
                nums2: vec![2,2],
                result: vec![2,2]
            },
            Case{
                nums1: vec![4,9,5],
                nums2: vec![9,4,9,8,4],
                result: vec![4,9]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::intersect(case_item.nums1.clone(), case_item.nums2.clone());
            assert_eq!(result, case_item.result)
        }
    }
}