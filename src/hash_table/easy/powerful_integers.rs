pub struct Solution {}

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut result:Vec<i32> = Vec::new();
        let mut i:i32 = 0;
        loop{
            if Self::pow(x, i) > bound{
                break
            }
            let mut j:i32 = 0;
            loop{
                let tmp = Self::pow(x, i) + Self::pow(y, j);
                if  tmp > bound {
                    break;
                } else if tmp <= bound && !result.contains(&tmp) {
                    result.push(tmp);
                }
                if y == 1 {
                    break
                }
                j += 1;
            }
            if x == 1 {
                break
            }
            i += 1
        }

        result
    }

    #[inline]
    fn pow(mut base: i32, mut exp: i32) -> i32 {
        if exp == 0 { return 1 }
        while exp & 1 == 0 {
            base = base * base;
            exp >>= 1;
        }
        if exp == 1 { return base }

        let mut acc = base;
        while exp > 1 {
            exp >>= 1;
            base = base * base;
            if exp & 1 == 1 {
                acc = acc * base;
            }
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        x: i32,
        y: i32,
        bound: i32,
        result: Vec<i32>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                x: 2,
                y: 3,
                bound: 10,
                result: vec![2,3,4,5,7,9,10]
            },
            Case{
                x: 3,
                y: 5,
                bound: 15,
                result: vec![2,4,6,8,10,14]
            },
            Case{
                x: 2,
                y: 1,
                bound: 10,
                result: vec![2,3,5,9]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::powerful_integers(case_item.x, case_item.y, case_item.bound);
            assert_eq!(result.len(), case_item.result.len());
            for item in result{
                assert_eq!(true, case_item.result.contains(&item));
            }
        }
    }
}