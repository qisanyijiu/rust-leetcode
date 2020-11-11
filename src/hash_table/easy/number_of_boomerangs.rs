pub struct Solution {}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut nums = 0;
        for p in points.iter(){
            let mut m:HashMap<i32, i32> = HashMap::with_capacity(points.len());
            for q in points.iter() {
                let key = (q[0] -p[0]).pow(2) + (q[1] - p[1]).pow(2);
                *m.entry(key).or_default() += 1;
            }
            nums += m.values().fold(0, |s, &v| s+ v * (v-1) )
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        points: Vec<Vec<i32>>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                points: vec![
                    vec![0,0],
                    vec![1,0],
                    vec![2,0]
                ],
                result: 2
            },
        ];
        for case_item in cases.iter(){
            let result = Solution::number_of_boomerangs(case_item.points.clone());
            assert_eq!(result, case_item.result)
        }
    }
}