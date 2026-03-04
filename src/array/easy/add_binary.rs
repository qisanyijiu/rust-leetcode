pub struct Solution {}

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let mut ans = Vec::new();
        let mut carry = 0;

        while !a.is_empty() || !b.is_empty() {
            let a = (a.pop() == Some('1')) as u8;
            let b = (b.pop() == Some('1')) as u8;
            ans.push((b'0' + (carry + a + b) % 2) as char);
            carry = (carry + a + b)/2;
        }
        if carry == 1  {
            ans.push('1');
        }
        ans.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        a: String,
        b: String,
        result: String,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                a: "11".to_string(),
                b: "1".to_string(),
                result: "100".to_string(),
            },
            Case {
                a: "1010".to_string(),
                b: "1011".to_string(),
                result: "10101".to_string(),
            },
        ];
        for case in cases {
            let result = Solution::add_binary(case.a, case.b);
            assert_eq!(result, case.result);
        }
    }
}