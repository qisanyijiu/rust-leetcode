pub struct Solution {}

impl Solution {
    pub fn ways_to_step(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut tmp1:i64 = 1;
        let mut tmp2:i64 = 2;
        let mut tmp3:i64 = 4;

        let mut x = 3;
        while x < n {
            let t3 = tmp3;
            tmp3 = tmp1 + tmp2 + tmp3;

            tmp1 = tmp2;
            tmp2 = t3;

            tmp1 = tmp1 % 1000000007;
            tmp2 = tmp2 % 1000000007;
            tmp3 = tmp3 % 1000000007;
            x += 1;
        }

        tmp3 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        n: i32,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                n: 1,
                result: 1
            },
            Case{
                n: 2,
                result: 2
            },
            Case{
                n: 3,
                result: 4
            },
            Case{
                n: 4,
                result: 7
            },
            Case{
                n: 5,
                result: 13
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::ways_to_step(case_item.n);
            assert_eq!(result, case_item.result)
        }
    }
}