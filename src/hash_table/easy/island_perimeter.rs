pub struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let row_nums = grid.len();
        let col_nums = grid[0].len();

        let mut result = 0;
        for (row_index, row) in grid.iter().enumerate(){
            for (col_index, place) in row.iter().enumerate(){
                if *place == 1 {
                    let mut p = 0;

                    //row_sub
                    if row_nums == 1 {
                        p += 2
                    }else{
                        if row_index == 0 {
                            //上边界
                            p += 1;
                            if grid[row_index+1][col_index] == 0 {
                                p += 1;
                            }

                        } else if row_index == row_nums - 1 {
                            //下边界
                            p += 1;
                            if grid[row_index-1][col_index] == 0 {
                                p += 1;
                            }
                        } else {
                            if grid[row_index+1][col_index] == 0 {
                                p += 1;
                            }
                            if grid[row_index-1][col_index] == 0 {
                                p += 1;
                            }
                        }
                    }
                    // col_sub
                    if  col_nums == 1{
                        p += 2;
                    }else{
                        if col_index == 0 {
                            // 左边界
                            p += 1;
                            if row[1] == 0 {
                                p += 1;
                            }
                        }else if col_index == col_nums - 1 {
                            // 右边界
                            p += 1;
                            if row[col_nums-2] == 0{
                                p += 1;
                            }

                        }else{
                            if row[col_index-1] == 0 {
                                p += 1;
                            }
                            if row[col_index+1] == 0 {
                                p += 1;
                            }

                        }
                    }
                    result += p;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        grid: Vec<Vec<i32>>,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                grid: vec![
                    vec![0,1,0,0],
                    vec![1,1,1,0],
                    vec![0,1,0,0],
                    vec![1,1,0,0]
                ],
                result: 16
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::island_perimeter(case_item.grid.clone());
            assert_eq!(result, case_item.result)
        }
    }
}