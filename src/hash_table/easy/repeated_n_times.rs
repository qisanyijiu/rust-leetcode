pub struct Solution {}

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();

        for elem in a {
            if !set.insert(elem) {
                return elem;
            }
        }
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        a: Vec<i32>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                a: vec![1,2,3,3],
                result: 3
            },
            Case{
                a: vec![2,1,2,5,3,2],
                result: 2
            },
            Case{
                a: vec![5,1,5,2,5,3,5,4],
                result: 5
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::repeated_n_times(case_item.a.clone());
            assert_eq!(result, case_item.result)
        }
    }
}