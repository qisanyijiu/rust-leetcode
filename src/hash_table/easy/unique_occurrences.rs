pub struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut count_map:HashMap<i32, i32> = HashMap::new();
        for item in arr.iter(){
            match count_map.get(item) {
                Some(cnt)=>{
                    count_map.insert(*item, *cnt + 1);
                },
                None=>{
                    count_map.insert(*item, 1);
                }
            }
        }
        let mut mem:Vec<i32> = vec![];
        for (_, v) in count_map.iter(){
            if !mem.contains(v){
                mem.push(*v)
            }else{
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        nums: Vec<i32>,
        result: bool
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                nums: vec![1,2,2,1,1,3],
                result: true
            },
            Case{
                nums: vec![1,2],
                result: false
            },
            Case{
                nums: vec![-3,0,1,-3,1,1,1,-3,10,0],
                result: true
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::unique_occurrences(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}