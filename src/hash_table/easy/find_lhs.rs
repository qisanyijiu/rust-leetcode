pub struct Solution {}

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut ma:HashMap<i32, i32> = HashMap::new();
        for num in nums.iter(){
            match ma.get(num) {
                Some(cnt)=>{
                    ma.insert(*num, *cnt+1);
                },
                None=>{
                    ma.insert(*num, 1);
                }
            }
        }
        let mut result = 0;
        for (k, v) in ma.iter(){
            let upper  = *k + 1;
            match ma.get(&upper) {
                Some(cnt) => {
                    if *v + *cnt > result{
                        result = *v + *cnt;
                    }
                },
                None => {}
            }
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
                nums: vec![1,3,2,2,5,2,3,7],
                result: 5
            },
            Case{
                nums: vec![1,2,2,2,2,3,3],
                result: 6
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_lhs(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}