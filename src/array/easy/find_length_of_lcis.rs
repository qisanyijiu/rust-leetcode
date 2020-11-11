pub struct Solution {}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() < 2{
            return nums.len() as i32
        }
        let mut result:i32 = 1;
        let mut i:usize =1;
        let mut sub_length: i32=1;
        while i < nums.len() {
            if nums[i]>nums[i-1]{
                sub_length+=1
            }
            if sub_length>result{
                result = sub_length
            }
            if nums[i]<=nums[i-1]{
                sub_length=1
            }
            i+=1;
        }
        result
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
            Case{
                nums: vec![1,3,5,4,7],
                result: 3
            },
            Case{
                nums: vec![2,2,2,2,2],
                result: 1
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_length_of_lcis(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}