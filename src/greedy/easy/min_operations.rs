pub struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut start_with_0 = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                if c == '1' {
                    start_with_0 += 1;
                }
            } else {
                if c == '0' {
                    start_with_0 += 1;
                }
            }
        }
        start_with_0.min(s.len() - start_with_0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        s: String,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{s: "0100".to_string(), result: 1},
            Case{s: "1001".to_string(), result: 2},
            Case{s: "1111".to_string(), result: 2},
        ];
        for case in cases {
            let result = Solution::min_operations(case.s);
            assert_eq!(result, case.result);
        }
    }
}