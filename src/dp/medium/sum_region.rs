/*
304. 二维区域和检索 - 矩阵不可变

https://leetcode-cn.com/problems/range-sum-query-2d-immutable/

给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (row1, col1) ，右下角为 (row2, col2)。

示例:

给定 matrix = [
  [3, 0, 1, 4, 2],
  [5, 6, 3, 2, 1],
  [1, 2, 0, 1, 5],
  [4, 1, 0, 1, 7],
  [1, 0, 3, 0, 5]
]

sumRegion(2, 1, 4, 3) -> 8
sumRegion(1, 1, 2, 2) -> 11
sumRegion(1, 2, 2, 4) -> 12
*/
struct NumMatrix {
    sum_matrix: Vec<Vec<i32>>
}

impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sum_matrix: Vec<Vec<i32>> = Vec::new();
        for (row_index, row) in matrix.into_iter().enumerate() {
            let mut current_row = Vec::new();
            for (col_index, col) in row.into_iter().enumerate() {
                if row_index == 0 && col_index == 0{
                    current_row.push(col)
                }else if row_index == 0 {
                    current_row.push(current_row[col_index-1] + col)
                }else if col_index == 0 {
                    current_row.push(sum_matrix[row_index-1][col_index] + col)
                }else{
                    current_row.push(current_row[col_index-1] + sum_matrix[row_index-1][col_index] + col - sum_matrix[row_index-1][col_index-1])
                }
            }
            sum_matrix.push(current_row);
        }
        NumMatrix{
            sum_matrix
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        if row1 == 0 && col1 == 0 {
            return self.sum_matrix[row2 as usize][col2 as usize]
        }else if row1 == 0 {
            return self.sum_matrix[row2 as usize][col2 as usize]- self.sum_matrix[row2 as usize][col1 as usize-1]
        }else if col1 == 0 {
            return self.sum_matrix[row2 as usize][col2 as usize]- self.sum_matrix[row1 as usize - 1][col2 as usize]
        }
        self.sum_matrix[row2 as usize][col2 as usize] + self.sum_matrix[row1 as usize - 1][col1 as usize - 1] - self.sum_matrix[row2 as usize][col1 as usize - 1] - self.sum_matrix[row1 as usize -1][col2 as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        row1: i32,
        col1: i32,
        row2: i32,
        col2: i32,
        result: i32,
    }

    #[test]
    fn test() {
        let num_matrix = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);

        let cases = vec![
            Case {
                row1: 2,
                col1: 1,
                row2: 4,
                col2: 3,
                result: 8,
            },
            Case {
                row1: 1,
                col1: 1,
                row2: 2,
                col2: 2,
                result: 11,
            },
            Case {
                row1: 1,
                col1: 2,
                row2: 2,
                col2: 4,
                result: 12,
            }
        ];
        for case_item in cases.iter() {
            let result = num_matrix.sum_region(case_item.row1, case_item.col1, case_item.row2, case_item.col2);
            assert_eq!(result, case_item.result)
        }
    }
}