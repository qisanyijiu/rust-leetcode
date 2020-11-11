pub struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut result:Vec<i32> = vec![0, 0];
        let mut map:Vec<i32> = vec![];

        for _ in &nums{
            map.push(0);
        }

        for num in &nums{
            let index = (num-1) as usize;
            map[index] += 1;
        }

        for (index, num) in map.iter().enumerate(){
            if *num == 2 {
                result[0] = (index + 1) as i32;
            }
            if *num == 0 {
                result[1] = (index + 1) as i32;
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
        result: Vec<i32>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                nums: vec![1,2,2,4],
                result: vec![2,3]
            },
            Case{
                nums: vec![1,3,3,4],
                result: vec![3,2]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_error_nums(case_item.nums.clone());
            assert_eq!(result, case_item.result)
        }
    }
}