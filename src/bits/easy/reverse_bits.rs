pub struct Solution {}

impl Solution {
    pub fn reverse_bits(mut n: u32) -> u32 {
        let mut rev_bin = Vec::with_capacity(32);

        while n > 0 {
            rev_bin.push(n % 2);

            n /= 2;
        }

        rev_bin.resize_with(32, || 0);

        let mut result = 0;

        for i in rev_bin {
            result = (result * 2) + i;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        n: u32,
        result: u32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                n: 43261596,
                result: 964176192,
            },
        ];
        for case in cases {
            let result = Solution::reverse_bits(case.n);
            assert_eq!(result, case.result);
        }
    }
}