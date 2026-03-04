pub struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut row_count: Vec<i32> = vec![0; mat.len()];
        let mut col_count: Vec<i32> = vec![0; mat[0].len()];
        let mut ans = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                row_count[i] += mat[i][j];
                col_count[j] += mat[i][j];
            }
        }
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if row_count[i] == 1 && col_count[j] == 1 && mat[i][j] == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        mat: Vec<Vec<i32>>,
        result: i32,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                mat: vec![vec![1,0,0], vec![0,0,1], vec![1,0,0]],
                result: 1,
            },
            Case {
                mat: vec![vec![1,0,0], vec![0,1,0], vec![0,0,1]],
                result: 3,
            },
        ];
        for case in cases {
            let result = Solution::num_special(case.mat);
            assert_eq!(result, case.result);
        }
    }
}