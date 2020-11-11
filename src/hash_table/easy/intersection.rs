pub struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut result = vec![];
        nums1.into_iter().for_each(|x| {set.insert(x);});
        nums2.into_iter().for_each(|x| {
            if set.contains(&x) {
                result.push(x.clone());
                set.remove(&x);
            }
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        result: Vec<i32>,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                nums1: vec![1,2,2,1],
                nums2: vec![2,2],
                result: vec![2],
            },
            Case {
                nums1: vec![4,9,5],
                nums2: vec![9,4,9,8,4],
                result: vec![9,4],
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::intersection(case_item.nums1.clone(), case_item.nums2.clone());
            assert_eq!(result, case_item.result)
        }
    }
}