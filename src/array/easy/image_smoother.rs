pub struct Solution {}

impl Solution {
    #![allow(unused_variables)]
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let width = m[0].len() - 1;
        let height = m.len() - 1;

        let mut output: Vec<Vec<i32>> = vec![];

        let mut row_index: usize = 0;
        while row_index <= height {
            let mut row: Vec<i32> = vec![];
            let mut col_index: usize = 0;
            while col_index <= width {
                let mut val: i32 = m[row_index][col_index];
                let mut n: i32 = 1;
                // 上
                if row_index>0 {
                    val += m[row_index-1][col_index];
                    n+=1;
                }
                // 下
                if row_index+1<=height {
                    val += m[row_index+1][col_index];
                    n+=1;
                }
                // 左
                if col_index>0{
                    val += m[row_index][col_index-1];
                    n+=1;
                }
                // 右
                if col_index+1<=width{
                    val += m[row_index][col_index+1];
                    n+=1;
                }
                // 左上
                if row_index>0 && col_index > 0{
                    val += m[row_index-1][col_index-1];
                    n+=1;
                }
                // 左下
                if row_index+1<=height && col_index > 0{
                    val += m[row_index+1][col_index-1];
                    n+=1;
                }
                // 右上
                if row_index>0 && col_index+1<=width{
                    val += m[row_index-1][col_index+1];
                    n+=1;
                }
                // 右下
                if row_index+1<=height && col_index+1<=width{
                    val += m[row_index+1][col_index+1];
                    n+=1;
                }
                row.push(val/n);
                col_index += 1;
            }
            output.push(row);
            row_index += 1
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        input: Vec<Vec<i32>>,
        result: Vec<Vec<i32>>,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                input: vec![
                    vec![1, 1, 1],
                    vec![1, 0, 1],
                    vec![1, 1, 1]
                ],
                result: vec![
                    vec![0, 0, 0],
                    vec![0, 0, 0],
                    vec![0, 0, 0]
                ],
            },
            Case {
                input: vec![
                    vec![2, 3, 4],
                    vec![5, 6, 7],
                    vec![8, 9, 10],
                    vec![11, 12, 13],
                    vec![14, 15, 16]
                ],
                result: vec![
                    vec![4,4,5],
                    vec![5,6,6],
                    vec![8,9,9],
                    vec![11,12,12],
                    vec![13,13,14]
                ],
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::image_smoother(case_item.input.clone());
            assert_eq!(result, case_item.result)
        }
    }
}