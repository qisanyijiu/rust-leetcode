pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut composite = vec![false; n];
        let mut count = 0;
        for i in 2..n {
            if !composite[i] {
                count += 1;
                let mut multiple = i * i; // Skip i * 2, i * 3, ... i * (i - 1), as they have already been marked.
                while multiple < n {
                    composite[multiple] = true;
                    multiple += i;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        n: i32,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                n: 3,
                result: 1,
            },
            Case {
                n: 10,
                result: 4,
            },
            Case {
                n: 100,
                result: 25,
            },
            Case {
                n: 499979,
                result: 41537,
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::count_primes(case_item.n);
            assert_eq!(result, case_item.result)
        }
    }
}